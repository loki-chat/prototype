use std::f32::consts::PI;

pub fn linear(t: f32) -> f32 {
	t
}

// sine

pub fn in_sine(t: f32) -> f32 {
	1. - (t * PI / 2.).cos()
}

pub fn out_sine(t: f32) -> f32 {
	((t * PI) / 2.).sin()
}

pub fn in_out_sine(t: f32) -> f32 {
	-((PI * t).cos() - 1.) / 2.
}

// quad

pub fn in_quad(t: f32) -> f32 {
	t.powi(2)
}

pub fn out_quad(t: f32) -> f32 {
	1. - (1. - t).powi(2)
}

pub fn in_out_quad(t: f32) -> f32 {
	if t < 0.5 {
		2. * t.powi(2)
	} else {
		1. - (-2. * t + 2.).powi(2) / 2.
	}
}

// cubic

pub fn in_cubic(t: f32) -> f32 {
	t.powi(3)
}

pub fn out_cubic(t: f32) -> f32 {
	1. - (1. - t).powi(3)
}

pub fn in_out_cubic(t: f32) -> f32 {
	if t < 0.5 {
		4. * t.powi(3)
	} else {
		1. - (-2. * t + 2.).powi(3) / 2.
	}
}

// quart

pub fn in_quart(t: f32) -> f32 {
	t.powi(4)
}

pub fn out_quart(t: f32) -> f32 {
	1. - (1. - t).powi(4)
}

pub fn in_out_quart(t: f32) -> f32 {
	if t < 0.5 {
		8. * t.powi(4)
	} else {
		1. - (-2. * t + 2.).powi(4) / 2.
	}
}

// quint

pub fn in_quint(t: f32) -> f32 {
	t.powi(5)
}

pub fn out_quint(t: f32) -> f32 {
	1. - (1. - t).powi(5)
}

pub fn in_out_quint(t: f32) -> f32 {
	if t < 0.5 {
		16. * t.powi(5)
	} else {
		1. - (-2. * t + 2.).powi(5) / 2.
	}
}

// expo

pub fn in_expo(t: f32) -> f32 {
	if t == 0. {
		0.
	} else {
		2_f32.powf(10. * t - 10.)
	}
}

pub fn out_expo(t: f32) -> f32 {
	if t == 1. {
		1.
	} else {
		1. - 2_f32.powf(-10. * t)
	}
}

pub fn in_out_expo(t: f32) -> f32 {
	if t == 0. {
		0.
	} else if t == 1. {
		1.
	} else if t < 0.5 {
		2_f32.powf(20. * t - 10.) / 2.
	} else {
		(2. - 2_f32.powf(-20. * t + 10.)) / 2.
	}
}

// circ

pub fn in_circ(t: f32) -> f32 {
	1. - (1. - t.powi(2)).sqrt()
}

pub fn out_circ(t: f32) -> f32 {
	(1. - (t - 1.).powi(2)).sqrt()
}

pub fn in_out_circ(t: f32) -> f32 {
	if t < 0.5 {
		(1. - (1. - (2. * t).powi(2)).sqrt()) / 2.
	} else {
		((1. - (-2. * t + 2.).powi(2)).sqrt() + 1.) / 2.
	}
}

// back

pub fn in_back(t: f32) -> f32 {
	let c1 = 1.70158;
	let c3 = c1 + 1.;
	c3 * t.powi(3) - c1 * t.powi(2)
}

pub fn out_back(t: f32) -> f32 {
	let c1 = 1.70158;
	let c3 = c1 + 1.;
	1. + c3 * (t - 1.).powi(3) + c1 * (t - 1.).powi(2)
}

pub fn in_out_back(x: f32) -> f32 {
	let c1 = 1.70158;
	let c2 = c1 * 1.525;

	if x < 0.5 {
		((2. * x).powi(2) * ((c2 + 1.) * 2. * x - c2)) / 2.
	} else {
		((2. * x - 2.).powi(2) * ((c2 + 1.) * (x * 2. - 2.) + c2) + 2.) / 2.
	}
}
