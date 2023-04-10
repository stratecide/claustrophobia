#import bevy_sprite::mesh2d_types
#import bevy_sprite::mesh2d_view_bindings

struct FragmentInput {
	#import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

	var color: f32 = 0.;
	let life_time_max = 10.;

	for (var i = 0.; i < 20.; i += 1.) {
		let rng = -1. + (i % 2.) * 2.;
		let cos_time = cos((globals.time + i + rng) * 2.);
		let life = 1. - ((globals.time / life_time_max + i / 20.) % 1.);
		let alpha = smoothstep(0., 2., life * life_time_max);
		let offset = 1. - life * (life + cos_time * 0.05) * 1.5;

		let edge = offset + cos(cos_time + in.uv.y * (15. + (i % 3.)) * rng + i) * 0.02;
		let edge_size = 0.08 * (life + 0.4);
		let col = smoothstep(edge - edge_size, edge + edge_size, in.uv.x)
			* smoothstep(edge + edge_size, edge - edge_size, in.uv.x);
		color = 1. - (1. - color) * (1. - col * alpha);
	}
	//color = smoothstep(0., 1., sqrt(color));

	// Output to screen
	return vec4<f32>(color, color / 6., color / 16., 1.0);

	//return vec4<f32>(in.uv, 0.5, smoothstep(1., 0.95, in.uv.x));
}

