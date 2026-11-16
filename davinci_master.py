import sys
import os
import ctypes
import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
from davinci_tracker import DavinciGameTracker

# ==========================================
# 1. NOTATION MODULE (SAN Conversion)
# ==========================================
def square_to_algebraic(sq):
    file = sq % 8
    rank = sq // 8
    return chr(ord('a') + file) + str(rank + 1)

def format_move_to_san(piece_type, from_sq, to_sq, captured=False, is_check=False, is_checkmate=False, promotion=None, is_castle=None):
    if is_castle == "kingside":
        san = "O-O"
    elif is_castle == "queenside":
        san = "O-O-O"
    else:
        piece_symbols = {0: '', 1: 'N', 2: 'B', 3: 'R', 4: 'Q', 5: 'K'}
        p_sym = piece_symbols.get(piece_type, '')
        dest = square_to_algebraic(to_sq)
        
        if piece_type == 0: 
            if captured:
                from_file = chr(ord('a') + (from_sq % 8))
                san = f"{from_file}x{dest}"
            else:
                san = f"{dest}"
        else:
            cap_str = "x" if captured else ""
            san = f"{p_sym}{cap_str}{dest}"
            
        if promotion:
            san += f"={promotion}"
            
    if is_checkmate:
        san += "#"
    elif is_check:
        san += "+"
        
    return san

# ==========================================
# 2. EVALUATION & MATERIAL MODULE (PST Removed)
# ==========================================
PIECE_VALUES = {
    'P': 100,  'N': 320,  'B': 330,  'R': 500,  'Q': 900,  'K': 20000,
    'p': -100, 'n': -320, 'b': -330, 'r': -500, 'q': -900, 'k': -20000
}

def evaluate_material(bitboards_array):
    score = 0
    piece_chars = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k']
    for idx, bb in enumerate(bitboards_array):
        count = bin(int(bb)).count('1')
        p_char = piece_chars[idx]
        if idx < 6:
            score += count * abs(PIECE_VALUES[p_char])
        else:
            score -= count * abs(PIECE_VALUES[p_char])
    return score

# ==========================================
# 3. NEURAL NETWORK (AI BRAIN) MODULE
# ==========================================
class DavinciEvaluationNet(nn.Module):
    def __init__(self):
        super(DavinciEvaluationNet, self).__init__()
        self.fc1 = nn.Linear(12 * 64, 256)
        self.fc2 = nn.Linear(256, 64)
        self.fc3 = nn.Linear(64, 1)

    def forward(self, x):
        x = F.relu(self.fc1(x))
        x = F.relu(self.fc2(x))
        score = torch.tanh(self.fc3(x))
        return score

# ==========================================
# 4. MASTER ENGINE CONTROLLER CLASS
# ==========================================
class DavinciMasterEngine:
    def __init__(self, lib_path="./libdavinci.so"):
        if not os.path.exists(lib_path):
            raise FileNotFoundError(f"{lib_path} not found! Compile the C++ bridge first.")
        self.lib = ctypes.CDLL(lib_path)
        
        self.lib.davinci_set_fen.argtypes = [ctypes.c_char_p]
        self.lib.davinci_validate_move.argtypes = [ctypes.c_int, ctypes.c_int, ctypes.c_int, ctypes.c_int]
        self.lib.davinci_validate_move.restype = ctypes.c_bool
        
        self.ai_brain = DavinciEvaluationNet()
        self.ai_brain.eval()
        
        # Integrate Game Tracker
        self.tracker = DavinciGameTracker()

    def set_position(self, fen):
        self.lib.davinci_set_fen(fen.encode('utf-8'))

    def get_board_tensor(self):
        bitboards_ptr = ctypes.cast(self.lib.davinci_bitboards, ctypes.POINTER(ctypes.c_uint64 * 12))
        bb_array = np.array(bitboards_ptr.contents, dtype=np.uint64)
        
        tensor_features = []
        for bb in bb_array:
            for i in range(64):
                bit = (int(bb) >> i) & 1
                tensor_features.append(float(bit))
                
        return torch.tensor(tensor_features, dtype=torch.float32).unsqueeze(0), bb_array

    def evaluate_position(self):
        """Evaluates position using Material + Neural Network (PST removed)"""
        tensor, bb_array = self.get_board_tensor()
        mat_score = evaluate_material(bb_array)
        
        with torch.no_grad():
            nn_score = self.ai_brain(tensor).item() * 100
            
        return mat_score + nn_score

    def play_and_record_move(self, piece_type, from_sq, to_sq, side, captured=False):
        """Validates move, records notation in tracker, and returns status"""
        is_legal = self.lib.davinci_validate_move(piece_type, from_sq, to_sq, side)
        if is_legal:
            san_move = format_move_to_san(piece_type, from_sq, to_sq, captured=captured)
            is_white = (side == 0)
            self.tracker.record_move(san_move, is_white_turn=is_white)
            return True, san_move
        return False, "ILLEGAL"
        
