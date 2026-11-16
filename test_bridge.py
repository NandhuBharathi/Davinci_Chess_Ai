import ctypes
import os

# Load the compiled C++ Shared Library (.so)
lib_path = "./libdavinci.so"
if not os.path.exists(lib_path):
    raise FileNotFoundError("libdavinci.so not found! Compile the C++ bridge first.")

davinci_lib = ctypes.CDLL(lib_path)

# Define function signatures with exact updated names from bridge.cpp
davinci_lib.davinci_set_fen.argtypes = [ctypes.c_char_p]
davinci_lib.davinci_validate_move.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int]
davinci_lib.davinci_validate_move.restype = ctypes.c_bool

# 1. Initialize starting FEN position using Davinci engine core bridge
start_fen = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
davinci_lib.davinci_set_fen(start_fen)

# 2. Test a legal move from Python: White Knight (piece_type=1) from B1 (sq 1) to C3 (sq 18)
is_legal = davinci_lib.davinci_validate_move(1, 1, 18, 0)
print(f"🐍 Python-Davinci Bridge Test [Knight B1 -> C3]: {'LEGAL ✅' if is_legal else 'ILLEGAL ❌'}")

# 3. Test an illegal move: White Pawn moving backward (A2 -> A1)
is_illegal = davinci_lib.davinci_validate_move(0, 8, 0, 0)
print(f"🐍 Python-Davinci Bridge Test [Pawn A2 -> A1]: {'LEGAL ✅ (Error)' if is_illegal else 'ILLEGAL ❌ (Correctly Blocked)'}")
