from flask import Flask, request, jsonify
import torch
import torch.nn as nn
import torch.optim as optim

app = Flask(__name__)

class Agent(nn.Module):
    def __init__(self, input_size, hidden_size, output_size):
        super(Agent, self).__init__()
        self.fc1 = nn.Linear(input_size, hidden_size)
        self.relu = nn.ReLU()
        self.fc2 = nn.Linear(hidden_size, output_size)

    def forward(self, x):
        x = self.fc1(x)
        x = self.relu(x)
        x = self.fc2(x)
        return x

def train_agent(agent, data, labels, epochs=1000, lr=0.01):
    loss_function = nn.CrossEntropyLoss()
    optimizer = optim.Adam(agent.parameters(), lr=lr)

    for epoch in range(epochs):
        optimizer.zero_grad()
        outputs = agent(data)
        loss = loss_function(outputs, labels)
        loss.backward()
        optimizer.step()
        if (epoch + 1) % 100 == 0:
            print(f'Epoch [{epoch+1}/{epochs}], Loss: {loss.item():.4f}')

def predict_action(agent, user_health, agent_health):
    input_data = torch.tensor([[user_health, agent_health]], dtype=torch.float)
    with torch.no_grad():
        output = agent(input_data)
        _, predicted = torch.max(output.data, 1)
        return predicted.item() + 1

@app.route('/predict', methods=['POST'])
def predict():
    user_health = int(request.form['user_health'])
    agent_health = int(request.form['agent_health'])

    action = predict_action(agent, user_health, agent_health)
    return jsonify({"action": action})

if __name__ == '__main__':
    input_size = 2
    hidden_size = 10
    output_size = 3
    data = torch.tensor([
        [100, 100],
        [50, 100],
        [100, 50],
        [10, 100],
        [100, 10],
        [50, 50]
    ], dtype=torch.float)
    labels = torch.tensor([2, 1, 1, 3, 1, 2], dtype=torch.long) - 1

    agent = Agent(input_size, hidden_size, output_size)
    train_agent(agent, data, labels, epochs=1000)

    app.run(host='0.0.0.0', port=8080, debug=True)
