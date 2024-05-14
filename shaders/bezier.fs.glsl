#version 460

in float vIntensity;

float colorIntensity() {
    return 1 - vIntensity*vIntensity*vIntensity*vIntensity;
    // exp(-10*pow(vIntensity,4.0));
}

out vec4 fFragColor;

void main(){
    fFragColor = vec4(1.)*colorIntensity();
    // vec4(1.0, 0.0, 0.0, 1.0);
}
