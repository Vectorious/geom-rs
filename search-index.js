var searchIndex = {};
searchIndex['geom'] = {"items":[[0,"","geom","",null,null],[0,"point","","",null,null],[3,"Point","geom::point","A generic two-dimensional point structure.",null,null],[8,"Position2D","","A type with `x` and `y` coordinates.",null,null],[10,"x","","Returns a copy of the `x` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[10,"y","","Returns a copy of the `y` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[10,"x_mut","","Returns a mutable reference to the `x` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[10,"y_mut","","Returns a mutable reference to the `y` coordinate of the position.",0,{"inputs":[{"name":"position2d"}],"output":{"name":"t"}}],[11,"fmt","","",1,{"inputs":[{"name":"point"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"bool"}}],[11,"clone","","",1,{"inputs":[{"name":"point"}],"output":{"name":"point"}}],[11,"new","","Returns a new `Point` with the given `x` and `y` coordinates.",1,{"inputs":[{"name":"point"},{"name":"t"},{"name":"t"}],"output":{"name":"point"}}],[11,"default","","Returns a new `Point` with the `x` and `y` coordinates being that of the default value of `T`.",1,{"inputs":[{"name":"point"}],"output":{"name":"point"}}],[11,"rect","","Returns a new `Rect` with the top-left point being the value of `self`\nand the bottom-right point being the value of `other`.",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"rect"}}],[11,"x","","Returns a copy of the `x` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[11,"y","","Returns a copy of the `y` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[11,"x_mut","","Returns a mutable reference to the `x` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[11,"y_mut","","Returns a mutable reference to the `x` coordinate of the point.",1,{"inputs":[{"name":"point"}],"output":{"name":"t"}}],[6,"Output","","",null,null],[11,"add","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"point"}}],[6,"Output","","",null,null],[11,"add","","",1,{"inputs":[{"name":"point"},{"name":"t"}],"output":{"name":"point"}}],[6,"Output","","",null,null],[11,"sub","","",1,{"inputs":[{"name":"point"},{"name":"point"}],"output":{"name":"point"}}],[6,"Output","","",null,null],[11,"sub","","",1,{"inputs":[{"name":"point"},{"name":"t"}],"output":{"name":"point"}}],[0,"rect","geom","",null,null],[3,"Rect","geom::rect","",null,null],[3,"Iter","","",null,null],[11,"fmt","","",2,{"inputs":[{"name":"rect"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"rect"}}],[11,"eq","","",2,{"inputs":[{"name":"rect"},{"name":"rect"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"rect"},{"name":"rect"}],"output":{"name":"bool"}}],[11,"new","","",2,{"inputs":[{"name":"rect"},{"name":"t"},{"name":"t"},{"name":"t"},{"name":"t"}],"output":{"name":"rect"}}],[11,"from_points","","",2,{"inputs":[{"name":"rect"},{"name":"point"},{"name":"point"}],"output":{"name":"rect"}}],[11,"width","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"height","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"top_left","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"point"}}],[11,"top","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"left","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"bottom_right","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"point"}}],[11,"bottom","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"right","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"area","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"t"}}],[11,"contains","","",2,{"inputs":[{"name":"rect"},{"name":"point"}],"output":{"name":"bool"}}],[11,"intersect","","",2,{"inputs":[{"name":"rect"},{"name":"rect"}],"output":{"name":"option"}}],[11,"split_column_mut","","",2,{"inputs":[{"name":"rect"},{"name":"t"}],"output":{"name":"rect"}}],[11,"split_row_mut","","",2,{"inputs":[{"name":"rect"},{"name":"t"}],"output":{"name":"rect"}}],[11,"columns","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"vec"}}],[11,"rows","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"vec"}}],[11,"iter","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"iter"}}],[6,"Item","","",null,null],[6,"IntoIter","","",null,null],[11,"into_iter","","",2,{"inputs":[{"name":"rect"}],"output":{"name":"iter"}}],[6,"Item","","",null,null],[11,"next","","",3,{"inputs":[{"name":"iter"}],"output":{"name":"option"}}],[6,"Point","geom","",null,null],[6,"Rect","","",null,null]],"paths":[[8,"Position2D"],[3,"Point"],[3,"Rect"],[3,"Iter"]]};
initSearch(searchIndex);