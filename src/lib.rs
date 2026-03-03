use lopdf::{Document, Object, ObjectId};
use std::collections::BTreeMap;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MergePdfError {
    #[error("Es wurde keine Eingabe-PDF uebergeben.")]
    EmptyInput,
    #[error("Datei existiert nicht oder ist kein gueltiger Pfad: {0}")]
    InvalidInputPath(String),
    #[error("Aus den Eingaben konnten keine gueltigen PDF-Root-Objekte aufgebaut werden.")]
    MissingRootObjects,
    #[error(transparent)]
    Pdf(#[from] lopdf::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn validate_input_paths<P: AsRef<Path>>(input_paths: &[P]) -> Result<(), MergePdfError> {
    if input_paths.is_empty() {
        return Err(MergePdfError::EmptyInput);
    }

    for path in input_paths {
        if !path.as_ref().is_file() {
            return Err(MergePdfError::InvalidInputPath(
                path.as_ref().display().to_string(),
            ));
        }
    }

    Ok(())
}

pub fn merge_pdfs<P: AsRef<Path>, Q: AsRef<Path>>(
    input_paths: &[P],
    output_path: Q,
) -> Result<(), MergePdfError> {
    validate_input_paths(input_paths)?;

    let mut max_id = 1;
    let mut page_object_ids = Vec::new();
    let mut documents_objects = BTreeMap::new();

    for input_path in input_paths {
        let mut document = Document::load(input_path)?;
        document.renumber_objects_with(max_id);
        max_id = document.max_id + 1;

        page_object_ids.extend(document.get_pages().into_values());
        documents_objects.extend(document.objects);
    }

    let mut document = Document::with_version("1.5");
    let mut catalog_object: Option<(ObjectId, Object)> = None;
    let mut pages_object: Option<(ObjectId, Object)> = None;

    for (object_id, object) in &documents_objects {
        match object.type_name().unwrap_or(b"") {
            b"Catalog" => {
                catalog_object = Some((*object_id, object.clone()));
            }
            b"Pages" => {
                if let Ok(dictionary) = object.as_dict() {
                    let has_pages = dictionary
                        .get(b"Count")
                        .and_then(Object::as_i64)
                        .map(|count| count > 0)
                        .unwrap_or(false);

                    if has_pages {
                        pages_object = Some((*object_id, object.clone()));
                    }
                }
            }
            b"Page" | b"Outlines" | b"Outline" => {}
            _ => {
                document.objects.insert(*object_id, object.clone());
            }
        }
    }

    let (pages_object_id, pages_object_value) =
        pages_object.ok_or(MergePdfError::MissingRootObjects)?;
    let (catalog_object_id, catalog_object_value) =
        catalog_object.ok_or(MergePdfError::MissingRootObjects)?;

    let mut pages_dictionary = pages_object_value.as_dict()?.clone();
    pages_dictionary.set("Count", page_object_ids.len() as u32);

    let kids = page_object_ids
        .iter()
        .map(|object_id| Object::Reference(*object_id))
        .collect::<Vec<_>>();
    pages_dictionary.set("Kids", kids);

    for object_id in &page_object_ids {
        if let Some(page_object) = documents_objects.get(object_id) {
            let mut page_object = page_object.clone();
            if let Ok(page_dictionary) = page_object.as_dict_mut() {
                page_dictionary.set("Parent", pages_object_id);
            }
            document.objects.insert(*object_id, page_object);
        }
    }

    let mut catalog_dictionary = catalog_object_value.as_dict()?.clone();
    catalog_dictionary.set("Pages", pages_object_id);

    document
        .objects
        .insert(pages_object_id, Object::Dictionary(pages_dictionary));
    document
        .objects
        .insert(catalog_object_id, Object::Dictionary(catalog_dictionary));

    document.trailer.set("Root", catalog_object_id);
    document.max_id = document.objects.len() as u32;
    document.renumber_objects();
    document.adjust_zero_pages();

    document.version = "1.5".to_string();

    document.compress();
    document.save(output_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lopdf::{dictionary, Dictionary, Stream};
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn build_sample_pdf(path: &Path, text: &str) -> Result<(), MergePdfError> {
        let mut document = Document::with_version("1.5");
        let pages_id = document.new_object_id();
        let page_id = document.new_object_id();
        let content_id = document.new_object_id();
        let font_id = document.new_object_id();
        let resources_id = document.new_object_id();
        let catalog_id = document.new_object_id();

        let content = format!("BT /F1 18 Tf 50 750 Td ({}) Tj ET", text);
        let stream = Stream::new(Dictionary::new(), content.into_bytes());

        document.objects.insert(content_id, Object::Stream(stream));
        document.objects.insert(
            font_id,
            dictionary! {
                "Type" => "Font",
                "Subtype" => "Type1",
                "BaseFont" => "Helvetica",
            }
            .into(),
        );
        document.objects.insert(
            resources_id,
            dictionary! {
                "Font" => dictionary! {
                    "F1" => font_id,
                },
            }
            .into(),
        );
        document.objects.insert(
            page_id,
            dictionary! {
                "Type" => "Page",
                "Parent" => pages_id,
                "MediaBox" => vec![0.into(), 0.into(), 595.into(), 842.into()],
                "Contents" => content_id,
                "Resources" => resources_id,
            }
            .into(),
        );
        document.objects.insert(
            pages_id,
            dictionary! {
                "Type" => "Pages",
                "Kids" => vec![page_id.into()],
                "Count" => 1,
            }
            .into(),
        );
        document.objects.insert(
            catalog_id,
            dictionary! {
                "Type" => "Catalog",
                "Pages" => pages_id,
            }
            .into(),
        );

        document.trailer.set("Root", catalog_id);
        document.compress();
        document.save(path)?;

        Ok(())
    }

    fn sample_paths(dir: &TempDir) -> (PathBuf, PathBuf, PathBuf) {
        let first = dir.path().join("first.pdf");
        let second = dir.path().join("second.pdf");
        let merged = dir.path().join("merged.pdf");
        (first, second, merged)
    }

    #[test]
    fn validate_inputs_rejects_empty() {
        let input: Vec<String> = vec![];
        let result = validate_input_paths(&input);
        assert!(matches!(result, Err(MergePdfError::EmptyInput)));
    }

    #[test]
    fn merge_outputs_document_with_all_pages() -> Result<(), MergePdfError> {
        let dir = TempDir::new()?;
        let (first, second, merged) = sample_paths(&dir);
        build_sample_pdf(&first, "Hallo 1")?;
        build_sample_pdf(&second, "Hallo 2")?;

        let files = vec![first, second];
        merge_pdfs(&files, &merged)?;

        let merged_doc = Document::load(&merged)?;
        assert_eq!(merged_doc.get_pages().len(), 2);
        Ok(())
    }
}
