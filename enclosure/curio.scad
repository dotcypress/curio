module Pcb(height=1.6) {
  length = 76.55;
  width = 25.451;

  translate([-length/2, -width/2, -height/2])
    hull()
      linear_extrude(height=height, convexity=8) 
      import(file="curio.svg");

  translate([-length/2 + 11.53, 0, 5])
    cube([10, 10, 10], center=true);
}

Pcb();