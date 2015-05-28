var searchIndex = {};
searchIndex['geom'] = {"items":[[0,"","geom","",null,null],[0,"point","","",null,null],[3,"Point","geom::point","A generic two-dimensional point structure.",null,null],[8,"Position2D","","A type with `x` and `y` coordinates.",null,null],[10,"x","","Returns a copy of the `x` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[10,"y","","Returns a copy of the `y` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[10,"x_mut","","Returns a mutable reference to the `x` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[10,"y_mut","","Returns a mutable reference to the `y` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[11,"fmt","","",1,{"inputs":[{"name":"point"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"bool"}}],[11,"clone","","",1,{"inputs":[{"name":"point"}],"output":{"name":"point"}}],[11,"new","","Returns a new `Point` with the given `x` and `y` coordinates.",1,{"inputs":[{"name":"point"},{"name":"t"},{"name":"t"}],"output":{"name":"point"}}],[11,"default","","Returns a new `Point` with the `x` and `y` coordinates being that of the default value of `T`.",1,{"inputs":[{"name":"point"}],"output":{"name":"point"}}],[11,"rect","","Returns a new `Rect` with the top-left point being the value of `self`\nand the bottom-right point being the value of `other`.",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"rect"}}],[11,"x","","Returns a copy of the `x` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[11,"y","","Returns a copy of the `y` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[11,"x_mut","","Returns a mutable reference to the `x` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[11,"y_mut","","Returns a mutable reference to the `x` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[11,"add","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"point"}}],[11,"add","","",1,{"inputs":[{"name":"point"},{"name":"t"}],"output":{"name":"point"}}],[11,"sub","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"point"}}],[11,"sub","","",1,{"inputs":[{"name":"point"},{"name":"t"}],"output":{"name":"point"}}],[0,"rect","geom","",null,null],[3,"Rect","geom::rect","A generic rectangle structure.",null,null],[3,"Iter","","An iterator over a rectangle, returning each point within the rectangle going from\nleft-to-right and top-to-bottom.",null,null],[11,"fmt","","",2,{"inputs":[{"name":"rect"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"rect"}}],[11,"eq","","",2,{"inputs":[{"name":"rect"},{"name":"rect"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"rect"},{"name":"rect"}],"output":{"name":"bool"}}],[11,"new","","Returns a new rectangle with the supplied position and dimensions.",2,{"inputs":[{"name":"rect"},{"name":"t"},{"name":"t"},{"name":"t"},{"name":"t"}],"output":{"name":"rect"}}],[11,"from_points","","Returns a new rectangle with the given top-left and bottom-right points.",2,{"inputs":[{"name":"rect"},{"name":"point"},{"name":"point"}],"output":{"name":"rect"}}],[11,"width","","Returns the width of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"height","","Returns the height of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"top_left","","Returns a copy of the top-left point of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"point"}}],[11,"top","","Returns the `y` coordinate of the top side of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"left","","Returns the `x` coordinate of the left side of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"bottom_right","","Returns a copy of the bottom-right point of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"point"}}],[11,"bottom","","Returns the `y` coordinate of the bottom side of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"right","","Returns the `x` coordinate of the right side of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"area","","Returns the area of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"contains","","Returns `true` if the given point lies within the bounds of the rectangle,\nand `false` otherwise.",2,{"inputs":[{"name":"rect"},{"name":"point"}],"output":{"name":"bool"}}],[11,"intersect","","If `self` and `other` intersect, then the intersection of the two rectangles\nis returned as a new rectangle, otherwise `None` is returned.",2,{"inputs":[{"name":"rect"},{"name":"rect"}],"output":{"name":"option"}}],[11,"split_column_mut","","Splits the rectangle at the given column `col`. The right side of the left part\nof the resulting split will be at `col - 1`, and the left side of the right part\nwill be at `col`. The current rectangle will be modified in-place to be the left\npart, and the right part will be returned as a new rectangle.",2,{"inputs":[{"name":"rect"},{"name":"t"}],"output":{"name":"rect"}}],[11,"split_row_mut","","Splits the rectangle at the given row `row`. The bottom side of the top part\nof the resulting split will be at `row - 1`, and the top side of the bottom part\nwill be at `row`. The current rectangle will be modified in-place to be the top\npart, and the bottom part will be returned as a new rectangle.",2,{"inputs":[{"name":"rect"},{"name":"t"}],"output":{"name":"rect"}}],[11,"columns","","Returns a `Vec` containing a one-width rectangle for each column of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"vec"}}],[11,"rows","","Returns a `Vec` containing a one-height rectangle for each row of the rectangle.",2,{"inputs":[{"name":"rect"}],"output":{"name":"vec"}}],[11,"iter","","Returns an iterator over each point in the rectangle, going from left-to-right and\ntop-to-bottom.",2,{"inputs":[{"name":"rect"}],"output":{"name":"iter"}}],[11,"into_iter","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"iter"}}],[11,"next","","",3,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[6,"Point","geom","",null,null],[6,"Rect","","",null,null]],"paths":[[8,"Position2D"],[3,"Point"],[3,"Rect"],[3,"Iter"]]};
initSearch(searchIndex);
