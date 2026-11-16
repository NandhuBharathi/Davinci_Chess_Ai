from davinci_master import DavinciMasterEngine

def run_engine_tests():
    print("🚀 Running Davinci Master Engine Test Suite...\n")
    
    # Initialize Engine
    engine = DavinciMasterEngine()
    
    # 1. Test FEN Setup & Evaluation
    start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    engine.set_position(start_fen)
    score = engine.evaluate_position()
    print(f"📊 Starting Position Evaluation Score: {score:.2f} centipawns")
    
    # 2. Test Legal Move: White Pawn e2 -> e4 (Square 12 to 28)
    # piece_type 0 = Pawn, from_sq = 12 (e2), to_sq = 28 (e4), side = 0 (White)
    is_l1, m1 = engine.play_and_record_move(piece_type=0, from_sq=12, to_sq=28, side=0, captured=False)
    print(f"♟️ Move 1 [White Pawn e2 -> e4]: [{m1}] -> {'LEGAL ✅' if is_l1 else 'ILLEGAL ❌'}")
    
    # 3. Test Legal Move: White Knight b1 -> c3 (Square 1 to 18)
    is_l2, m2 = engine.play_and_record_move(piece_type=1, from_sq=1, to_sq=18, side=0, captured=False)
    print(f"♟️ Move 2 [White Knight b1 -> c3]: [{m2}] -> {'LEGAL ✅' if is_l2 else 'ILLEGAL ❌'}")
    
    # Print tracked game history log
    engine.tracker.print_board_history()
    
    print("✨ All Pawn & Piece Tests Completed Successfully!")

if __name__ == "__main__":
    run_engine_tests()
    
