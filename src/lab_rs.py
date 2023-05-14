import ctypes

# Load the Rust dylib
rust_lib = ctypes.CDLL('../target/release/lab_rs.dll')  # Replace with the actual dylib path

# Define the struct Entry
class Entry(ctypes.Structure):
    _fields_ = [
        ('name', ctypes.c_char_p),
        ('raster', ctypes.c_char_p),
    ]

# Define the struct LabFile
class LabFile(ctypes.Structure):
    pass

# LabFile function prototypes
rust_lib.LabFile_new.argtypes = [ctypes.c_char_p]
rust_lib.LabFile_new.restype = ctypes.POINTER(LabFile)

rust_lib.LabFile_add_label.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p, ctypes.c_char_p]
rust_lib.LabFile_add_label.restype = None

rust_lib.LabFile_add_ramcell.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p, ctypes.c_char_p]
rust_lib.LabFile_add_ramcell.restype = None

rust_lib.LabFile_del.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p]
rust_lib.LabFile_del.restype = None

rust_lib.LabFile_write.argtypes = [ctypes.POINTER(LabFile), ctypes.c_char_p]
rust_lib.LabFile_write.restype = ctypes.c_int

rust_lib.LabFile_read_from_file.argtypes = [ctypes.c_char_p]
rust_lib.LabFile_read_from_file.restype = ctypes.POINTER(LabFile)
