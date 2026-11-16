
class DavinciGameTracker:
    def __init__(self):
        self.move_history = []
        self.half_move_clock = 0
        self.full_move_number = 1

    def record_move(self, san_move, is_white_turn=True):
        """Records a validated SAN move into the game history log"""
        if is_white_turn:
            move_entry = f"{self.full_move_number}. {san_move}"
            self.move_history.append(move_entry)
        else:
            if self.move_history:
                self.move_history[-1] += f" {san_move}"
            self.full_move_number += 1
            
        self.half_move_clock += 1

    def get_pgn_game_string(self):
        """Returns the full game notation string (PGN format)"""
        return " ".join(self.move_history)

    def print_board_history(self):
        """Prints the tracked game movement log"""
        print("\n📜 --- Davinci Game History & Position Tracking ---")
        print(self.get_pgn_game_string())
        print("--------------------------------------------------\n")
        
