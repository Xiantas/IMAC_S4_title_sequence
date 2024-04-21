#version 460

in float vIntensity;

float colorIntensity(){
    return exp(-10*pow(vIntensity,4.0));
}

out vec4 fFragColor;

void main(){
    fFragColor = vec4(vec3(1.,0.,0.)*colorIntensity(),1.);
    // vec4(1.0, 0.0, 0.0, 1.0);
}
