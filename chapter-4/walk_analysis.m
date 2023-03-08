clear all; clc; close all;

walks = readmatrix("./data/walk/walks.csv");

L = size(walks);
L = L(1);

figure(1);
grid on;
hold on;
view(3);

for i = 1:3:L
    disp(i);
    x = walks(i, :);
    y = walks(i + 1, :);
    z = walks(i + 2, :);
    plot3(x, y, z, "MarkerFaceColor", [i / L, 0, 0]);
end