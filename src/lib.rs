use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::ops::Add;

enum Section {
	SETTINGS,
	LABEL,
	RAMCELL,
}

#[derive(PartialEq, Debug)]
struct Entry {
    name: String,
    raster: String,
}

impl Entry {
    fn new(name: String, raster: String) -> Entry {
        Entry{name, raster}
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.name, self.raster)
    }
}

#[derive(PartialEq, Debug)]
pub struct LabFile {
	settings: String,
    label: HashMap<String, Entry>,
	ramcell: HashMap<String, Entry>,
}

impl LabFile {
	pub fn new(header: &str) -> LabFile {
		let new_header: String = String::from(header.trim());
		let new_label: HashMap<String, Entry> = HashMap::new();
		let new_ramcell: HashMap<String, Entry> = HashMap::new();
        LabFile{settings: new_header, label: new_label, ramcell: new_ramcell}
    }
    pub fn add_label(&mut self, label_name: &str, label_raster: &str) {
		let new_entry: Entry = Entry::new(String::from(label_name), String::from(label_raster));
        self.label.insert(new_entry.name.clone(), new_entry);
    }
	pub fn add_ramcell(&mut self, label_name: &str, label_raster: &str) {
		let new_entry: Entry = Entry::new(String::from(label_name), String::from(label_raster));
        self.ramcell.insert(new_entry.name.clone(), new_entry);
    }
	pub fn write(&mut self, filename: &str) -> Result<(), Box<dyn Error>> {
		let mut output: File = File::create(filename)?;
		write!(output, "{}", self.to_string())?;
		Ok(())
	}

	pub fn read_from_file(filename: &str) -> Result<LabFile, Box<dyn Error>> {
		let file: File = File::open(filename)?;
		let reader: BufReader<File> = BufReader::new(file);
		let mut lab_file: LabFile = LabFile::new("");
		let mut sec: Section = Section::SETTINGS;
		
		for line in reader.lines() {
			let line: String = line?;
			if line.is_empty() {
				// pass
			} else if line.starts_with("[SETTINGS]") {
				sec = Section::SETTINGS;
			} else if line.starts_with("[LABEL]") {
				sec = Section::LABEL;
			} else if line.starts_with("[RAMCELL]") {
				sec = Section::RAMCELL;
			} else {
				let fields: Vec<&str> = line.split(';').collect();
				match sec {
					Section::SETTINGS => {
						lab_file.settings.push_str(&String::from(line.trim()));
						lab_file.settings.push_str("\n");
					},
					Section::LABEL => {
						if fields.len() > 1 {
							lab_file.add_label(fields[0].trim(), fields[1].trim());
						} else {
							lab_file.add_label(fields[0].trim(), "");
						}
					},
					Section::RAMCELL => {
						if fields.len() > 1 {
							lab_file.add_ramcell(fields[0].trim(), fields[1].trim());
						} else {
							lab_file.add_ramcell(fields[0].trim(), "");
						}
					}
				} 
			}
		}
		Ok(lab_file)
	}
}

impl Add for LabFile {
	type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut outp_lab = LabFile::new(&self.settings);
		if !self.label.is_empty() {
			for val in self.label.values() {
				outp_lab.add_label(&val.name, &val.raster);
			}
		}
		if !self.ramcell.is_empty() {
			for val in self.ramcell.values() {
				outp_lab.add_ramcell(&val.name, &val.raster);
			}
		}
		if !other.label.is_empty() {
			for val in other.label.values() {
				outp_lab.add_label(&val.name, &val.raster);
			}
		}
		if !other.ramcell.is_empty() {
			for val in other.ramcell.values() {
				outp_lab.add_ramcell(&val.name, &val.raster);
			}
		}
		return outp_lab;
    }
}

impl fmt::Display for LabFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut outp = format!("[SETTINGS]\n{}\n", self.settings);
		if !self.label.is_empty() {
			outp.push_str("[LABEL]\n");
            for val in self.label.values() {
                let entry: String;
				if val.raster.is_empty() {
					entry = format!("{};\n", val.name);
				} else {
					entry = format!("{}; {};\n", val.name, val.raster);
				}
                outp.push_str(&entry);
            }
			outp.push_str("\n");
        }
		if !self.ramcell.is_empty() {
			outp.push_str("[RAMCELL]\n");
            for val in self.ramcell.values() {
				let entry: String;
				if val.raster.is_empty() {
					entry = format!("{};\n", val.name);
				} else {
					entry = format!("{}; {};\n", val.name, val.raster);
				}
                outp.push_str(&entry);
            }
			//outp.push_str("\n");
        }
		write!(f, "{}", outp.trim_end_matches(", "))
    }
}

/* fn main() -> Result<(), Box<dyn Error>> {
    let lab_file = read_lab_file("entries.csv")?;
    println!("{}", lab_file);
    Ok(())
} */


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn new_entry () {
		let entry1: Entry = Entry::new(String::from("test_name"), String::from("test_raster"));
		let entry2: Entry = Entry {name: String::from("test_name"), raster: String::from("test_raster")};
		assert_eq!(entry1, entry2);
	}

	#[test]
	fn print_entry () {
		let entry2: Entry = Entry {name: String::from("test_name"), raster: String::from("test_raster")};
		assert_eq!(entry2.to_string(), "(test_name, test_raster)");
	}
}
