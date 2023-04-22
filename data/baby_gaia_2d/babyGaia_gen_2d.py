import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
import numpy as np
from ezkl import export
import torch
import torch.nn as nn
import torch.optim as optim
from torch.utils.data import DataLoader, TensorDataset
import numpy as np

torch.set_printoptions(profile='full')


class AutonomousWorldModel(nn.Module):
    def __init__(self, input_size, hidden_size, output_size):
        super(AutonomousWorldModel, self).__init__()
        self.fc1 = nn.Linear(input_size, hidden_size)
        self.fc2 = nn.Linear(hidden_size, hidden_size)
        self.fc3 = nn.Linear(hidden_size, hidden_size)
        self.fc4 = nn.Linear(hidden_size, output_size)
        self.relu = nn.ReLU()
        self.dropout = nn.Dropout(0.5)

    def forward(self, x):
        x = self.relu(self.fc1(x))
        x = self.dropout(x)
        x = self.relu(self.fc2(x))
        x = self.dropout(x)
        x = self.relu(self.fc3(x))
        x = self.dropout(x)
        x = self.fc4(x)
        return x


def create_datasets():
    # Create dummy data for demonstration purposes
    data_size = 1000
    state_size = 12 * 12 * 2  # Two channels for each cell
    input_size = 2 * state_size
    output_size = 12 * 12 * 2  # Updated output size
    num_elements = 24

    previous_states = torch.randint(0, num_elements, (data_size, state_size)).float()
    current_states = torch.randint(0, num_elements, (data_size, state_size)).float()
    output_boards = torch.randint(0, num_elements, (data_size, output_size)).float()

    # Concatenate previous_states and current_states along the second dimension
    input_states = torch.cat((previous_states, current_states), dim=1)

    dataset = TensorDataset(input_states, output_boards)
    return dataset


def train_test_split(dataset, test_split=0.2):
    test_size = int(len(dataset) * test_split)
    train_size = len(dataset) - test_size
    train_dataset, test_dataset = torch.utils.data.random_split(dataset, [train_size, test_size])
    return train_dataset, test_dataset


def main():
    device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')

    # Create dataset and train/test split
    dataset = create_datasets()
    train_dataset, test_dataset = train_test_split(dataset)

    train_loader = DataLoader(train_dataset, batch_size=32, shuffle=True)
    test_loader = DataLoader(test_dataset, batch_size=32, shuffle=False)

    # Define model, loss function, and optimizer
    input_size = 2 * (12 * 12 * 2)
    hidden_size = 128
    output_size = 12 * 12 * 2  # Updated output size
    model = AutonomousWorldModel(input_size, hidden_size, output_size).to(device)
    criterion = nn.MSELoss()
    optimizer = optim.Adam(model.parameters(), lr=0.001)
    num_epochs = 100

    # Training loop
    for epoch in range(num_epochs):
        for (inputs, correct_output_board) in train_loader:
            inputs = inputs.to(device)
            correct_output_board = correct_output_board.to(device)
            outputs = model(inputs)
            loss = criterion(outputs, correct_output_board)

            optimizer.zero_grad()
            loss.backward()
            optimizer.step()

        # # Testing loop
        # with torch.no_grad():
        #     for (inputs, correct_output_board) in test_loader:
        #         inputs = inputs.to(device)
        #         correct_output_board = correct_output_board.to(device)
        #         outputs = model(inputs)

        #         # Print the outputs and ground truth in 12 * 12 * 2 format
        #         for i in range(inputs.shape[0]):
        #             print("Predicted output board:")
        #             predicted_board = outputs[i].view(12, 12, 2)
        #             print(predicted_board.cpu().numpy())

        #             print("Ground truth board:")
        #             ground_truth_board = correct_output_board[i].view(12, 12, 2)
        #             print(ground_truth_board.cpu().numpy())

        #             print("\n")

    export(model, input_shape=[2 * 12 * 12 * 2])

if __name__ == '__main__':
    main()

