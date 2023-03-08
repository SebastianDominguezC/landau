i = readmatrix("./data/prng/i.csv");
r = readmatrix("./data/prng/r.csv");
f = readmatrix("./data/prng/f.csv");
s = readmatrix("./data/prng/s.csv");

figure(1);
plot(i, r);
title('lazy power residue')
xlabel('iterations')
ylabel('prn')

figure(2);
plot(i, f);
title('rusts prng')
xlabel('iterations')
ylabel('prn')

figure(3);
plot(i, s);
title('stronger power residue')
xlabel('iterations')
ylabel('prn')

j = length(i);

x = zeros(1, j);
y = zeros(1, j);
for k = 1:j / 2
    x(k) = r(2 * k - 1);
    y(k) = r(2 * k);
end

figure(4);
scatter(x, y);
title('r periodicity')
xlabel('x(r)')
ylabel('y(r)')

x = zeros(1, j);
y = zeros(1, j);
for k = 1:j / 2
    x(k) = f(2 * k - 1);
    y(k) = f(2 * k);
end

figure(5);
scatter(x, y);
title('f periodicity')
xlabel('x(r)')
ylabel('y(r)')

x = zeros(1, j);
y = zeros(1, j);
for k = 1:j / 2
    x(k) = s(2 * k - 1);
    y(k) = s(2 * k);
end

figure(6);
scatter(x, y);
title('s periodicity')
xlabel('x(r)')
ylabel('y(r)')

