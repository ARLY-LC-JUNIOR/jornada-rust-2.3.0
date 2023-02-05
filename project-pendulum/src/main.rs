use speedy2d::color::Color;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};

fn main() {
    //Precisamos deste objeto janela para criar uma janela.
    let window = Window::new_centered("Pendulum", (800, 480)).unwrap();

    //Precisamos criar um ajudante de janela para lidar com os eventos de janela.
    let win = MyWindowHandler {
        p: Pendulum::new(400.0, 0.0, 200.0),
        p2: Pendulum::new(400.0, 0.0, 400.0),
    };

    //Run the loop.
    window.run_loop(win);
}

//Este é o manipulador de janela.
//Trata os eventos da janela e tem os objetos que queremos desenhar e a lógica.
struct MyWindowHandler {
    p: Pendulum,
    p2: Pendulum,
}

//Precisamos implementar o trait WindowHandler para nosso manipulador de janela.
impl WindowHandler for MyWindowHandler {
    //A função de desenho é chamada a cada quadro.
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        //Precisamos limpar a tela a cada quadro.
        graphics.clear_screen(Color::from_rgb(0.8, 0.9, 1.0));

        //Desenhando o primeiro pêndulo.
        self.p.update();
        self.p.draw(graphics);

        //Precisamos desenhar o primeiro pêndulo.
        self.p2.update();
        self.p2.draw(graphics);

        //Draw the frame.
        helper.request_redraw();
    }
}

////Esta é a estrutura do pêndulo.
struct Pendulum {
    //Este vetor é a posição do pêndulo.
    origin: vector::Vector,

    //Este vetor é a posição da bola.
    position: vector::Vector,

    //Este é o ângulo do pêndulo.
    angle: f32,

    angular_velocity: f32,
    angular_acceleration: f32,

    r: f32, //The length of the pendulum.
    m: f32, //The mass of the ball.
    g: f32, //The gravity.
}

//Precisamos implementar a estrutura do pêndulo.
impl Pendulum {
    //Este é o construtor do pêndulo.
    //Toma a posição x e y do pêndulo e o comprimento do pêndulo.
    fn new(x: f32, y: f32, r: f32) -> Pendulum {
        //Precisamos devolver o pêndulo.
        Pendulum {
            //We need to set the origin of the pendulum.
            origin: vector::Vector::new(x, y),

            //Definiremos a posição quando atualizarmos o pêndulo.
            //Por enquanto vamos defini-lo como um valor padrão.
            position: vector::Vector::new(0.0, 0.0),

            angle: 1.0,                //We'll set the angle to 1.0 radian.
            angular_velocity: 0.0,     //The pendulum is not moving in the beginning.
            angular_acceleration: 0.0, //The pendulum is not accelerating in the beginning.

            r: r,
            m: 1.0, //A massa da bola é 1,0 para este exemplo.
            g: 0.5, //A gravidade é 0.5 para este exemplo, mas brinque com ela.
        }
    }

    //Esta função atualiza o pêndulo a cada quadro.
    fn update(&mut self) {
        //We use the pendulum equation to calculate the angular acceleration.
        self.angular_acceleration = -1.0 * self.g * self.angle.sin() / self.r;

        //The angular velocity is the angular velocity plus the angular acceleration.
        self.angular_velocity += self.angular_acceleration;

        //The angle is the angle plus the angular velocity.
        self.angle += self.angular_velocity;

        //The posisition is the polar coordinates translated to cartesian coordinates.
        self.position
            .set(self.r * self.angle.sin(), self.r * self.angle.cos());

        //The final position of the ball in the canvas is the origin of the
        //pendulum plus the position vector.
        self.position.add(&self.origin);
    }

    //This function draws the pendulum. It takes the graphics object as a parameter.
    fn draw(&self, graphics: &mut Graphics2D) {
        //Precisamos primeiro traçar a linha do pêndulo.
        //Pega a posição inicial e final da linha, a largura da linha e a cor.
        graphics.draw_line(
            (self.origin.x, self.origin.y),
            (self.position.x, self.position.y),
            3.0,
            Color::GREEN,
        );

        //We need to draw the ball of the pendulum.
        //It takes the position of the ball, the radius of the ball and the color.
        graphics.draw_circle((self.position.x, self.position.y), 30.0, Color::BLACK);
    }
}

//This is a module that contains the vector struct.
pub mod vector {
    //This is the vector struct that we use for the position of the pendulum and the ball.
    pub struct Vector {
        pub x: f32,
        pub y: f32,
    }

    //The vector implementation.
    impl Vector {
        //The constructor of the vector.
        pub fn new(x: f32, y: f32) -> Vector {
            Vector { x, y } //We return a new vector.
        }

        //Esta função adiciona um vetor ao vetor atual.
        //Modificamos o vetor atual.
        //Outra opção seria retornar um novo vetor.
        pub fn add(&mut self, other: &Vector) -> &Vector {
            self.x += other.x;
            self.y += other.y;
            self
        }

        //This function sets the x and y values of the vector.
        pub fn set(&mut self, x: f32, y: f32) {
            self.x = x;
            self.y = y;
        }
    }
}