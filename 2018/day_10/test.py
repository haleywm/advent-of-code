import re
P = []
for line in open('input.txt'):
    x,y,vx,vy = map(int, re.findall('-?\d+', line))
    P.append([x,y,vx,vy])

for t in range(100000):
    min_x = min([x for x,y,_,_ in P])
    max_x = max([x for x,y,_,_ in P])
    min_y = min([y for x,y,_,_ in P])
    max_y = max([y for x,y,_,_ in P])
    W = 100
    if min_x+W >= max_x and min_y + W >= max_y:
        #print t,min_x, max_x, min_y, max_y
        print(t)
        for y in range(min_y, max_y+1):
            for x in range(min_x, max_x+1):
                if (x,y) in [(px,py) for px,py,_,_ in P]:
                    print('#', end="")
                else:
                    print('.', end="")
            print()

    for p in P:
        p[0] += p[2]
        p[1] += p[3]