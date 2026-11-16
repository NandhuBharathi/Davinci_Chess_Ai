import torch
import torch.nn as nn
import torch.nn.functional as F

class DavinciEvaluationNet(nn.Module):
    def __init__(self):
        super(DavinciEvaluationNet, self).__init__()
        # Input: 12 bitboards (each 8x8 flattened or processed via CNN/Linear layers)
        # For simplicity and lightning-fast inference, we use a fully connected architecture first
        self.fc1 = nn.Linear(12 * 64, 256)
        self.fc2 = nn.Linear(256, 64)
        self.fc3 = nn.Linear(64, 1)  # Outputs a single scalar evaluation score (-1.0 to 1.0)

    def forward(self, x):
        # x shape: [Batch_size, 768] (12 pieces * 64 squares)
        x = F.relu(self.fc1(x))
        x = F.relu(self.fc2(x))
        score = torch.tanh(self.fc3(x)) # Bounded score between -1 and 1
        return score

# Test the Neural Network structure
if __name__ == "__main__":
    net = DavinciEvaluationNet()
    print("🧠 Davinci AI Brain (PyTorch Model) Initialized Successfully!")
    
    # Dummy board state tensor (Batch of 1, 768 features)
    dummy_board = torch.randn(1, 768)
    eval_score = net(dummy_board)
    print(f"📊 Sample Position Evaluation Score: {eval_score.item():.4f}")
    
