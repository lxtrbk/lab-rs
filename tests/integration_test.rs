use lab_rs::LabFile;

#[test]
fn integration() {
    let mut lbl_file = LabFile::new("test_setting");
    lbl_file.add_label("a_lbl_name", "a_lbl_raster");
    lbl_file.add_ramcell("b_ram_name", "b_ram_raster");
    lbl_file.add_ramcell("c_ram_name", "c_ram_raster");
    println!("{}", lbl_file);
}

#[test]
fn integ_from_file() {

}