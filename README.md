# trilateration

The tool to make svg with triangles by length from two points.

```text
0 0 2500 2500 10
# header line.
# <origin in svg x> <origin in svg y> <width> <height> <scale in svg>

# <variable> = <func>(<args>)
# <func>(<args>)

# func `xy`: xy(i32, i32) -> point
# create point i32, i32 and returns point
a = xy(0, 0)
b = xy(18450, 0)
# func `lplp`: lplp(i32, point, i32, point): 
# create a point far first i32 from first point
# and from second i32 from second one.
c = lplp(19120, a, 05005, b)

# func `line`: line(point, point)
# draw line from first point to second point
line(a, b)

```
