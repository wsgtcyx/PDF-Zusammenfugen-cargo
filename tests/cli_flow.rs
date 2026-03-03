use lopdf::{dictionary, Dictionary, Document, Object, Stream};
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn build_sample_pdf(path: &Path, text: &str) -> Result<(), Box<dyn std::error::Error>> {
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

#[test]
fn cli_merges_two_valid_pdfs() -> Result<(), Box<dyn std::error::Error>> {
    let dir = TempDir::new()?;
    let first = dir.path().join("first.pdf");
    let second = dir.path().join("second.pdf");
    let merged = dir.path().join("merged.pdf");

    build_sample_pdf(&first, "Hallo 1")?;
    build_sample_pdf(&second, "Hallo 2")?;

    let bin = env!("CARGO_BIN_EXE_pdf-zusammenfugen-cargo");
    let output = Command::new(bin)
        .arg("-o")
        .arg(&merged)
        .arg(&first)
        .arg(&second)
        .output()?;

    assert!(
        output.status.success(),
        "CLI fehlgeschlagen: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let merged_doc = Document::load(&merged)?;
    assert_eq!(merged_doc.get_pages().len(), 2);

    Ok(())
}
