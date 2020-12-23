import numpy as np
import scipy.ndimage as img

DIMS = (3,  4)

def main():
    for dims in DIMS:
        cube = np.array([[x == "#" for x in line.strip()] for line in open("input.txt")])
        cube = cube.reshape((cube.shape[0], cube.shape[1]) + (1,) * (dims - 2))

        comp = np.ones((3,) * dims).astype(np.uint32)
        comp[(1,) * dims] = 0

        for _ in range(6):
            cube = np.pad(cube, 1).astype(np.uint32)
            # Get adjacent with a convolution
            adj = img.correlate(cube, comp, mode="constant", cval=0)
            # Doing some fancy bitwise stuff
            cube = ((cube == 0) & (adj == 3)) | ((cube == 1) & ((adj == 3) | (adj == 2)))

        print(cube.sum())

if __name__ == "__main__":
    main()