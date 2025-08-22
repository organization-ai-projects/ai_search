use ndarray::Array1;

pub fn sigmoid(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| 1.0 / (1.0 + (-v).exp()))
}

pub fn dsigmoid(y: &Array1<f64>) -> Array1<f64> {
    y * &(1.0 - y)
}

pub fn to_array(v: &[f64]) -> Array1<f64> {
    Array1::from(v.to_vec())
}

// Activation fractale inspirée : sinusoïde récursive (exemple simple)
pub fn fractal_activation(x: &Array1<f64>, depth: usize) -> Array1<f64> {
    if depth == 0 {
        x.mapv(|v| v)
    } else {
        let prev = fractal_activation(x, depth - 1);
        prev.mapv(|v| (v * 3.0).sin())
    }
}

// Dérivée approximative pour la version sinusoïdale
pub fn dfractal_activation(x: &Array1<f64>, depth: usize) -> Array1<f64> {
    if depth == 0 {
        x.mapv(|_| 1.0)
    } else {
        let prev = fractal_activation(x, depth - 1);
        let dprev = dfractal_activation(x, depth - 1);
        prev.mapv(|v| 3.0 * v.cos()) * dprev
    }
}

// Activation mycélienne : tanh(sin(x))
pub fn mycelium_activation(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| (v.sin()).tanh())
}

// Dérivée de tanh(sin(x))
pub fn dmycelium_activation(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| {
        let s = v.sin();
        let t = s.tanh();
        (1.0 - t * t) * v.cos()
    })
}

// Activation pulsative : onde carrée lissée (tanh(10*sin(x)))
pub fn pulsative_activation(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| (10.0 * v.sin()).tanh())
}

// Dérivée de tanh(10*sin(x))
pub fn dpulsative_activation(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| {
        let s = (10.0 * v.sin()).tanh();
        (1.0 - s * s) * 10.0 * v.cos()
    })
}

// Activation logistique normalisée : f(x) = r·x·(1−x) avec x∈[0,1] (on applique sigmoid)
pub fn logistic_activation(x: &Array1<f64>) -> Array1<f64> {
    let r = 4.0;
    let x_norm = sigmoid(x);
    x_norm.mapv(|v| r * v * (1.0 - v))
}

// Dérivée de f(x) = r·x·(1−x) par rapport à x (en tenant compte de la normalisation)
pub fn dlogistic_activation(x: &Array1<f64>) -> Array1<f64> {
    let r = 4.0;
    let x_norm = sigmoid(x);
    // d/dx [r·σ(x)·(1−σ(x))] = r·σ'(x)·(1−2σ(x))
    let ds = x_norm.mapv(|v| 1.0 - v) * &x_norm; // σ(x)·(1−σ(x))
    let d_sigma = ds; // σ'(x)
    d_sigma * (1.0 - 2.0 * &x_norm) * r
}

// Activation "trou noir doux" : gaussienne centrée f(x) = exp(-x^2)
pub fn soft_blackhole_activation(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| (-v * v).exp())
}

// Dérivée de f(x) = exp(-x^2) : f'(x) = -2x * exp(-x^2)
pub fn dsoft_blackhole_activation(x: &Array1<f64>) -> Array1<f64> {
    x.mapv(|v| -2.0 * v * (-v * v).exp())
}
