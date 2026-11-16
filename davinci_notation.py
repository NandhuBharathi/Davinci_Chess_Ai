
def square_to_algebraic(sq):
    """Converts a square index (0 to 63) to chess coordinate string (e.g., 0 -> 'a1', 63 -> 'h8')"""
    file = sq % 8
    rank = sq // 8
    return chr(ord('a') + file) + str(rank + 1)

def format_move_to_san(piece_type, from_sq, to_sq, captured=False, is_check=False, is_checkmate=False, promotion=None, is_castle=None):
    """
    Converts move details into Standard Algebraic Notation (SAN).
    piece_type: 0=Pawn, 1=Knight, 2=Bishop, 3=Rook, 4=Queen, 5=King
    """
    # 1. Castling check
    if is_castle == "kingside":
        san = "O-O"
    elif is_castle == "queenside":
        san = "O-O-O"
    else:
        # Piece symbols (Pawns have no symbol in notation)
        piece_symbols = {0: '', 1: 'N', 2: 'B', 3: 'R', 4: 'Q', 5: 'K'}
        p_sym = piece_symbols.get(piece_type, '')
        
        dest = square_to_algebraic(to_sq)
        
        # 2. Pawn moves & captures (e.g., e4 or exd5)
        if piece_type == 0: 
            if captured:
                from_file = chr(ord('a') + (from_sq % 8))
                san = f"{from_file}x{dest}"
            else:
                san = f"{dest}"
        else:
            # 3. Piece moves & captures (e.g., Nf3 or Nxf3)
            cap_str = "x" if captured else ""
            san = f"{p_sym}{cap_str}{dest}"
            
        # 4. Promotion handling (e.g., e8=Q)
        if promotion:
            san += f"={promotion}"
            
    # 5. Game status suffix (+ for check, # for checkmate)
    if is_checkmate:
        san += "#"
    elif is_check:
        san += "+"
        
    return san

"""if __name__ == "__main__":
    print("📝 Davinci Notation Module Initialized Successfully!\n")
    
    # Test cases:
    # 1. Knight B1 to C3 -> Nc3
    print(f"Test 1 (Knight Move): {format_move_to_san(1, 1, 18)}")
    
    # 2. Pawn captures on d5 from e4 -> exd5
    print(f"Test 2 (Pawn Capture): {format_move_to_san(0, 28, 35, captured=True)}")
    
    # 3. Queen captures on f7 with checkmate -> Qxf7#
    print(f"Test 3 (Checkmate Move): {format_move_to_san(4, 59, 53, captured=True, is_checkmate=True)}")
    
    # 4. Kingside Castling -> O-O
    print(f"Test 4 (Castling): {format_move_to_san(5, 4, 6, is_castle='kingside')}")"""
    
