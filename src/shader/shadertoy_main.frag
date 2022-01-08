layout(location = 0) out vec4 out_color;

vec3 srgb2lin(vec3 cs) {
	vec3 c_lo = cs / 12.92;
	vec3 c_hi = pow((cs + 0.055) / 1.055, vec3(2.4));
	vec3 s = step(vec3(0.04045), cs);
	return mix(c_lo, c_hi, s);
}

void main() {
	vec4 fragColor;
	vec2 fragCoord = vec2(gl_FragCoord.x, iResolution.y-gl_FragCoord.y);
	mainImage(fragColor, fragCoord);

	// shadertoy expects to output colors in sRGB, but we write to the swapchain
	// which is a `*UnormSrgb` texture so it will perform the linear -> sRGB conversion for us.
	out_color = vec4(srgb2lin(fragColor.rgb), fragColor.a);
}
