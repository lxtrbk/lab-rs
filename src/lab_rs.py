import ctypes

# Load the Rust .dll
rust_lib = ctypes.CDLL('../target/release/lab_rs.dll')

# Define the Rust struct Entry
class Entry(ctypes.Structure):
    _fields_ = [
        ('name', ctypes.c_char_p),
        ('raster', ctypes.c_char_p),
    ]

# Define the Rust struct LabFile
class LabFile(ctypes.Structure):
    _fields_ = [
        ('settings', ctypes.c_char_p),
        ('label', {}),
        ('ramcell', {}),
    ]

# LabFile function prototypes
rust_lib.LabFile_new.argtypes = [ctypes.c_char_p]
rust_lib.LabFile_new.restype = ctypes.POINTER(LabFile)

rust_lib.LabFile_add_label.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p, ctypes.c_char_p]
rust_lib.LabFile_add_label.restype = None

rust_lib.LabFile_add_ramcell.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p, ctypes.c_char_p]
rust_lib.LabFile_add_ramcell.restype = None

rust_lib.LabFile_delete_label.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p]
rust_lib.LabFile_delete_label.restype = None

rust_lib.LabFile_write.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p]
rust_lib.LabFile_write.restype = ctypes.c_int

rust_lib.LabFile_read_from_file.argtypes = [ctypes.c_char_p]
rust_lib.LabFile_read_from_file.restype = ctypes.POINTER(LabFile)

# Define the Python class Entry
class Entry:
    def __init__(self, name, raster):
        self.name = name
        self.raster = raster

# Define the Python class LabFile
class LabFile:
    def __init__(self, header):
        self.lab_file = rust_lib.LabFile_new(header.encode())

    def add_label(self, label_name, label_raster):
        rust_lib.LabFile_add_label(self.lab_file, label_name.encode(), label_raster.encode())

    def add_ramcell(self, label_name, label_raster):
        rust_lib.LabFile_add_ramcell(self.lab_file, label_name.encode(), label_raster.encode())

    def delete_label(self, label_name):
        rust_lib.LabFile_delete_label(self.lab_file, label_name.encode())

    def write_to_file(self, filename):
        result = rust_lib.LabFile_write(self.lab_file, filename.encode())
        if result == 0:
            print("LabFile written successfully")
        else:
            print("Failed to write LabFile")

    @staticmethod
    def read_from_file(filename):
        lab_file_ptr = rust_lib.LabFile_read_from_file(filename.encode())
        if lab_file_ptr:
            lab_file = lab_file_ptr.contents
            # Create Python Entry objects from Rust Entry objects
            label_dict = {
                name.decode(): Entry(name.decode(), raster.decode())
                for name, raster in lab_file.label.items()
            }
            ramcell_dict = {
                name.decode(): Entry(name.decode(), raster.decode())
                for name, raster in lab_file.ramcell.items()
            }

            # Create the LabFile object
            lab_file_obj = LabFile(lab_file.settings)
            lab_file_obj.label = label_dict
            lab_file_obj.ramcell = ramcell_dict

            return lab_file_obj

        return None

