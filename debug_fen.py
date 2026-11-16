import ctypes
import os
import numpy as np

lib_path = "./libdavinci.so"
if not os.path.exists(lib_path):
    print("❌ libdavinci.so not found! Compile the C++ bridge first.")
else:
    lib = ctypes.CDLL(lib_path)
    lib.davinci_set_fen.argtypes = [ctypes.c_char_p]
    
    # Set standard starting FEN
    start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    lib.davinci_set_fen(start_fen.encode('utf-8'))
    
    # Fetch bitboards array
    bitboards_ptr = ctypes.cast(lib.davinci_bitboards, ctypes.POINTER(ctypes.c_uint64 * 12))
    bb_array = np.array(bitboards_ptr.contents, dtype=np.uint64)
    
    print("🔍 --- Automatic FEN Parser & Square Mapping Debugger ---")
    
    # Check White Pawns Bitboard (Index 0)
    white_pawns_bb = bb_array[0]
    print(f"White Pawns Bitboard (Hex): {hex(white_pawns_bb)}")
    
    # Find which square indices have bits set to 1
    set_squares = []
    for sq in range(64):
        if (white_pawns_bb >> sq) & 1:
            file = sq % 8
            rank = sq // 8
            alg = chr(ord('a') + file) + str(rank + 1)
            set_squares.append(f"{alg} (Index {sq})")
            
    print(f"Pawns detected by C++ at squares: {set_squares}")
    print("---------------------------------------------------------")
    
