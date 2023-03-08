close all; clc; clear all;
data = readmatrix("./data/decay/data.csv");

t = data(1, :);
deltas = data(2, :);
N = data(3, :);

figure(1);
plot(t, log(N));
ylabel("log(N) particles");
xlabel("t");
title("Particle decay along time");
