// 名前付きフィード構造体

struct Polygon{
    vertexes: Vec<(i32, i32)>,
    //頂点の座標
    stroke_width: u8,
    // 輪郭の太さ
    fill:(u8, u8, u8),
    //塗り潰しのRGB色
}

// タプル構造体
struct Triangle(Vertex, Vertex, Vertex);
struct Vertex(i32, i32);

//ユニット構造体
struct UniqueValue;

fn new_polygon(vertexes: Vec<(i32, i32)>) -> Polygon{
    let stroke_width = 1;
    let fill = (0, 0, 0);
    Polygon{vertexes, stroke_width, fill}
}


//または
// struct UniqueValue{}
// struct UniqueValue();
fn main(){
    // polygon 型の値を作り、変数 triangle を束縛する
    let triangle = Polygon{
        vertexes: vec![(0,0), (3,0), (2, 2)],
        fill: (255, 255, 255),
        stroke_width: 1,
    };
    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    assert_eq!(triangle.vertexes[0], (0,0));
    assert_eq!(triangle.vertexes.len(), 3);
    assert_eq!(triangle.fill, (255, 255, 255));
    
    //パターンマッチでアクセス。不要なフィードは..で省略できる
     let Polygon {vertexes: quad_vx, ..} = quadrangle;
     assert_eq!(4, quad_vx.len());

     // : 以降を省略すると、フィードと同じ名前の変数が作られ
     //  フィールド値に束縛される

     let Polygon{ fill, ..} = quadrangle;
     assert_eq!((0, 0, 0), fill);


     //構造体の値を変更するにはmutが必要
     let mut polygon = new_polygon(vec![(-1, 5), (-4, 0)]);
     assert_eq!(polygon.vertexes.len(), 2);
     polygon.vertexes.push((2, 8));
     assert_eq!(polygon.vertexes.len(), 3);

     let triangle1 = Polygon{
         vertexes: vec![(0,0), (3, 0), (2, 2)],
         fill: (255, 255, 255),
         stroke_width: 5,
     };

     // triangle1 を元にvertexesだけ異なる新しい値を作る
     let triangele2 = Polygon{
         vertexes: vec![(0,0), (-3, 0), (-2, 2)],
         .. triangle1
     };


}