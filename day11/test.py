import random

my_divisors = [7, 19, 5, 4]
factor = 1
for f in my_divisors:
    factor *= f


real_v = 23
mod_v = 23
for i in range(60):
    c = random.randint(0, 3)
    if c == 0:
        v = random.randint(0, 20)
        real_v += v
        mod_v += v
    elif c == 1:
        v = random.randint(0, 20)
        real_v *= v
        mod_v *= v
    elif c == 2:
        v = random.randint(0, 20)
        real_v *= real_v
        mod_v *= mod_v

    mod_v = mod_v % factor

    for div in my_divisors:
        print((mod_v % div), (real_v % div))
        assert (mod_v % div) == (real_v % div)
