clc; close all;
clear all;

fname = 'data/protein/fold.json'; 
fid = fopen(fname); 
raw = fread(fid,inf); 
str = char(raw'); 
data = jsondecode(str);
fclose(fid); 

connections = readmatrix('data/protein/connections.csv');

n = length(data);

figure(1);
hold on;

% Plot points
for i = 1 : n
    for j = 1 : n
        for k = 1 : n
            if data(i, j, k) == 0
                plot3(i, j, k, '.', 'Color', '#efefef', 'MarkerSize', 15);
            end
            if data(i, j, k) == 1
                plot3(i, j, k, '.', 'Color', 'blue', 'MarkerSize', 15);
            end
            if data(i, j, k) == 2
                plot3(i, j, k, '.', 'Color', 'red', 'MarkerSize', 15);
            end 
        end
    end
end

% Plot connections
l = length(connections);

for i = 1 : l
    xi = connections(i, 1) + 1;
    yi = connections(i, 2) + 1;
    zi = connections(i, 3) + 1;
   
    xf = connections(i, 4) + 1;
    yf = connections(i, 5) + 1;
    zf = connections(i, 6) + 1;
    plot3([xi, xf], [yi, yf], [zi, zf], 'Color', 'black');
end

xlabel('x');
ylabel('y');
zlabel('z');
title("Protein chain");

