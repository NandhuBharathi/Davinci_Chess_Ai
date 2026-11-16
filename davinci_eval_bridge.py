import ctypes
import os
import torch
import numpy as np
from davinci_nn import DavinciEvaluationNet

# 1. Load C++ Shared Library
lib_path = "./libdavinci.so"
if not os.path.exists(lib_path):
    raise FileNotFoundError("libdavinci.so not found!")

davinci_lib = ctypes.CDLL(lib_path)

# Define pointer type for 12 U64 bitboards
davinci_lib.davinci_set_fen.argtypes = [ctypes.c_char_p]

# Helper function to extract bitboards from C++ into a PyTorch tensor
def get_board_tensor(lib):
    # Get pointer to global davinci_bitboards array in C++ (12 uint64_t values)
    bitboards_ptr = ctypes.cast(lib.davinci_bitboards, ctypes.POINTER(ctypes.c_uint64 * 12))
    bb_array = np.array(bitboards_ptr.contents, dtype=np.uint64)
    
    # Convert each 64-bit integer into 64 binary float features (0 or 1)
    tensor_features = []
    for bb in bb_array:
        for i in range(64):
            bit = (bb >> i) & 1
            tensor_features.append(float(bit))
            
    # Reshape to PyTorch expected shape: [1, 768] (12 pieces * 64 squares)
    return torch.tensor(tensor_features, dtype=torch.float32).unsqueeze(0)

# 2. Initialize AI Model
ai_brain = DavinciEvaluationNet()
ai_brain.eval() # Evaluation mode

# 3. Set FEN in C++ Engine and Evaluate via PyTorch
start_fen = b"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
davinci_lib.davinci_set_fen(start_fen)

# Extract tensor from C++ engine state
board_tensor = get_board_tensor(davinci_lib)

# Predict score using PyTorch model
with torch.no_grad():
    evaluation = ai_brain(board_tensor)

print(f"🔗 C++ & PyTorch Bridge Integration Successful!")
print(f"♟️ Starting Position AI Evaluation Score: {evaluation.item():.4f}")
