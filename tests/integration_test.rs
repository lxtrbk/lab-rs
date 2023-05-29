use lab_rs::LabFile;
use std::fs;

// test create lab file representation
#[test]
fn lab_creation() {
    let mut lbl_file: LabFile = LabFile::new("test_setting\n");
    lbl_file.add_label("a_lbl_name", "a_lbl_raster");
    lbl_file.add_ramcell("b_ram_name", "b_ram_raster");
    lbl_file.add_ramcell("c_ram_name", "c_ram_raster");
    lbl_file.add_ramcell("d_ram_name", "");
    //println!("{}", lbl_file);
}

// test read from file

// test display

// test write to file
#[test]
fn disk_read_write() {
    let mut lbl_file1 = LabFile::read_from_file(r#"tests/debug_test_in.lab"#).unwrap();
    lbl_file1.write(r#"tests/debug_test_out.lab"#).unwrap();
    //println!("{}", lbl_file1);
    let lbl_file2 = LabFile::read_from_file(r#"tests/debug_test_out.lab"#).unwrap();
    assert_eq!(lbl_file1, lbl_file2);
    fs::remove_file(r#"tests/debug_test_out.lab"#).unwrap();
    //println!("end");
}
