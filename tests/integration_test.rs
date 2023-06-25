use lab_rs::LabFile;
use std::fs;

// test add label and display function
#[test]
fn lab_creation() {
    let mut lbl_file: LabFile = LabFile::new("test_setting\n");
    lbl_file.add_label("a_lbl_name", "a_lbl_raster");
    lbl_file.add_ramcell("b_ram_name", "b_ram_raster");
    lbl_file.add_ramcell("c_ram_name", "c_ram_raster");
    lbl_file.add_ramcell("d_ram_name", "");
    assert_eq!(lbl_file.to_string(), String::from("[SETTINGS]\ntest_setting\n\n[LABEL]\na_lbl_name; a_lbl_raster;\n\n[RAMCELL]\nb_ram_name; b_ram_raster;\nc_ram_name; c_ram_raster;\nd_ram_name;\n\n"))
}

// test delete label
#[test]
fn test_del() {
    let mut file1: LabFile = LabFile::new("test_setting\n");
    file1.add_label("a_lbl_name", "a_lbl_raster");
    file1.add_ramcell("b_ram_name", "b_ram_raster");
    file1.delete_label("a_lbl_name");
    file1.delete_label("b_ram_name");
    let file2: LabFile = LabFile::new("test_setting\n");
    assert_eq!(file1, file2);
}

// test read/write from file
#[test]
fn disk_read_write() {
    let mut lbl_file1: LabFile = LabFile::read_from_file(r#"tests/file1.lab"#).unwrap();
    lbl_file1.write(r#"tests/file_out.lab"#).unwrap();
    //println!("{}", lbl_file1);
    let lbl_file2: LabFile = LabFile::read_from_file(r#"tests/file_out.lab"#).unwrap();
    assert_eq!(lbl_file1, lbl_file2);
    fs::remove_file(r#"tests/file_out.lab"#).unwrap();
    //println!("end");
}

// test add function
#[test]
fn test_add() {
    let file1: LabFile = LabFile::read_from_file(r#"tests/file1.lab"#).unwrap();
    let file2: LabFile = LabFile::read_from_file(r#"tests/file2.lab"#).unwrap();
    let file3: LabFile = LabFile::read_from_file(r#"tests/file3.lab"#).unwrap();
    let file4: LabFile = file1 + file2;
    assert_eq!(file3, file4);
}

// test sub function
#[test]
fn test_sub() {
    let file1: LabFile = LabFile::read_from_file(r#"tests/file1.lab"#).unwrap();
    let file2: LabFile = LabFile::read_from_file(r#"tests/file2.lab"#).unwrap();
    let file3: LabFile = LabFile::read_from_file(r#"tests/file3.lab"#).unwrap();
    let file4: LabFile = file3 - file2;
    assert_eq!(file4, file1);
}
