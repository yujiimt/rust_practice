// デカルト座標

struct CartesianCoord{
    x: f64, 
    y: f64
}

struct PolarCoord{
    r: f64,
    theta: f64,
}

// 座標
trait Coordinates{
    // 関数の本体は書かない
    fn to_cartesian(self) -> CartesianCoord;
    fn from_cartesian(cart: CartesianCoord) -> Self;
}

// トレイトをそれぞれの型に実装する

// デカルト座標はそのまま
impl Coordinates for CartesianCoord{
    fn to_cartesian(self) -> CartesianCoord{
        self
    }
    fn from_cartesian(cart: CartesianCoord) ->Self{
        cart
    }
}

// 極座標は変換が必要
impl Coordinates for PolarCoord{
    fn to_cartesian(self) -> CartesianCoord{
        CartesianCoord{
            x: self.r * self.theta.cos(),
            y: self.r * self.theta.sin(),
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self{
        PolarCoord{
            r: (cart.x * cart.x + cart.y * cart.y).sqrt(),
            theta: (cart.y / cart.x).atan(),
        }
    }
}

//タプルにもトレイトを実装できる
impl Coordinates for (f64, f64){
    fn to_cartesian(self) -> CartesianCoord{
        CartesianCoord{
            x: self.0,
            y: self.1,
        }
    }
    fn from_cartesian(cart: CartesianCoord) -> Self{
        (cart.x, cart.y)
    }
}


fn print_point(point: impl Coordinates){
    let p = point.to_cartesian();
    println!("({}, {})", p.x, p.y)
}
//線形変換に必要な行列を定義しておく
struct Matrix([[f64; 2]; 2]);
//座標に対して線形変換を定義する
trait LinearTransform: Coordinates{
    fn transform(self, matrix: &Matrix) -> Self;
    
    fn rotate(self, theta: f64) -> Self
    where
        Self: Sized,
        {
            self.transform(&Matrix([
                [theta.cos(), -theta.sin()],
                [theta.sin(), theta.cos()],
            ]))
        }
}

impl LinearTransform for CartesianCoord{
    fn transform(mut self, matrix: &Matrix) -> Self{
        let x = self.x;
        let y = self.y;
        let m = matrix.0;

        self.x = m[0][0] * x + m[0][1] * y;
        self.y = m[1][0] * x + m[1][1] * y;
        self
    }
}


fn main(){
    let point = (1.0, 1.0);

    // トレイトのメソッドを呼ぶ
    let c = point.to_cartesian();
    println!("x = {}, y = {}", c.x, c.y);


    // 同じくトレイトの関連関数を呼ぶ
    let p = PolarCoord::from_cartesian(c);
    println!("r = {}, theta = {}", p.r, p.theta);

    print_point((0.0, 1.0));
    print_point(PolarCoord{
        r: 1.0,
        theta: std::f64::consts::PI / 2.0,
    });

    let p2 = (1.0, 0.0).to_cartesian();
    print_point(p2.rotate(std::f64::consts::PI)); 
    
}