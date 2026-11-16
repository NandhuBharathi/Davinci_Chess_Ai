import torch
import numpy as np

# Standard Chess Piece Values in Centipawns
PIECE_VALUES = {
    'P': 100,  'N': 320,  'B': 330,  'R': 500,  'Q': 900,  'K': 20000,
    'p': -100, 'n': -320, 'b': -330, 'r': -500, 'q': -900, 'k': -20000
}

# Simple Material Evaluation based on C++ Bitboards state
def evaluate_material(bitboards_array):
    """
    bitboards_array: Array of 12 U64 integers from C++ bridge
    [0-5: White P, N, B, R, Q, K | 6-11: Black p, n, b, r, q, k]
    """
    score = 0
    piece_chars = ['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k']
    
    for idx, bb in enumerate(bitboards_array):
        # Count bits set to 1 (number of pieces)
        count = bin(bb).count('1')
        p_char = piece_chars[idx]
        
        # Add or subtract based on piece value
        if idx < 6: # White pieces
            score += count * abs(PIECE_VALUES[p_char])
        else: # Black pieces
            score -= count * abs(PIECE_VALUES[p_char])
            
    return score

if __name__ == "__main__":
    print("♟️ Davinci Chess Knowledge & Piece Valuation Module Ready!")
    
    # Test with a sample bitboards array (Starting position counts without ULL suffix)
    sample_bbs = [
        0x00ff000000000000, # P
        0x4200000000000000, # N
        0x2400000000000000, # B
        0x8100000000000000, # R
        0x0800000000000000, # Q
        0x1000000000000000, # K
        # Black pieces mirror
        0x000000000000ff00, # p
        0x0000000000000042, # n
        0x0000000000000024, # b
        0x0000000000000081, # r
        0x0000000000000008, # q
        0x0000000000000010  # k
    ]
    
    mat_score = evaluate_material(sample_bbs)
    print(f"⚖️ Starting Position Material Balance Score: {mat_score} centipawns (Balanced)")
    
