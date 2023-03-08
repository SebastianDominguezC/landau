clc; close all;
clear all;

data = readmatrix('data/protein/fold.csv');
connections = readmatrix('data/protein/connections.csv');

n = length(data);

figure(1);
hold on;

% Plot points
for i = 1 : n
    for j = 1 : n
        if data(i, j) == 0
            plot(i, j, '.', 'Color', '#efefef', 'MarkerSize', 15);
        end
        if data(i, j) == 1
            plot(i, j, '.', 'Color', 'blue', 'MarkerSize', 15);
        end
        if data(i, j) == 2
            plot(i, j, '.', 'Color', 'red', 'MarkerSize', 15);
        end
        
    end
end

% Plot connections
l = length(connections);

for i = 1 : l
    xi = connections(i, 1) + 1;
    xf = connections(i, 3) + 1;
    yi = connections(i, 2) + 1;
    yf = connections(i, 4) + 1;
    plot([xi, xf], [yi, yf], 'Color', 'black');
end

title("Protein chain");

