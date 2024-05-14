#version 460 core
layout(location=0) in int aTruc;
layout(location=1) in vec2 aP0;
layout(location=2) in vec2 aP1;
layout(location=3) in vec2 aP2;
layout(location=4) in vec2 aP3;

uniform int uTotalVerticesCount;

out float vIntensity;

const vec2 bP0 = vec2(-0.5, 0.5);
const vec2 bP1 = vec2(0.5, 0.5);
const vec2 bP2 = vec2(-0.5, -0.5);
const vec2 bP3 = vec2(0.5, -0.5);


const vec2 corones[4] = vec2[4](
    vec2(0.5, 0.5),
    vec2(-0.5, 0.5),
    vec2(0.5, -0.5),
    vec2(-0.5, -0.5)
);

vec2 polynomeBezier(float t){
    return aP0*pow(1-t,3) + 
           aP1*3*t*pow(1-t,2) + 
           aP2*3*pow(t,2)*(1-t) + 
           aP3*pow(t,3);
}


vec2 deriveBezier(float t){
    return aP0*(-3)*pow(1-t,2) +
                  aP1*(3*pow(1-t,2)-6*t*(1-t)) + 
                  aP2*(6*t*(1-t)-3*pow(t,2)) +
                  aP3*3*pow(t,2);
}

vec2 normalBezier(float t){
    vec2 normal = deriveBezier(t);
    return vec2(normal.y*pow(-1,gl_VertexID%2),normal.x*pow(-1,(gl_VertexID%2)+1));
}

vec2 polyBez(float t) {
    return (-aP0 + 3*aP1 - 3*aP2 + aP3) * t*t*t +
        (3*aP0 - 6*aP1 + 3*aP2) * t*t +
        (-3*aP0 + 3*aP1) * t + aP0;
}

vec2 deriBez(float t) {
    return 3*(-aP0 + 3*aP1 - 3*aP2 + aP3) * t*t +
        2*(3*aP0 - 6*aP1 + 3*aP2) * t +
        (-3*aP0 + 3*aP1);
}

vec2 normaBez(float t) {
    vec2 deri = normalize(deriBez(t))/10.0;
    if(gl_VertexID%2 == 0){
        return vec2(deri.y, -deri.x);    
    }
    else{
        return vec2(-deri.y, deri.x);
    }    
}

float width(float t){
    if (t < 0.1) {
        return t*t*10.0;
    } else if (t > 0.9) {
        return (1-t)*(1-t)*10.0;
    } else {
        return 0.1;
    }
    // exp(-10*pow(t*2-1.0,10.0))/2;
}

void main(){
    float t = float(gl_VertexID) / float(uTotalVerticesCount-1);
    /*
    if(gl_VertexID == 0 || gl_VertexID == uTotalVerticesCount-1){
        vIntensity = 0.0;
    }
    else
    */
    if (gl_VertexID % 2 == 0) {
        vIntensity = 1.0;
    } else {
        vIntensity = -1.0;
    }

    vec2 pos = polyBez(t) + width(t) * normaBez(t);

/*
    if (length(pos) > 1.0 || length(pos) < 0.05) {
        pos = normalize(pos);
    }
*/

    gl_Position = vec4(pos, 0.0, 1.0);
    // vec4(corones[gl_VertexID%4], 0.0, 1.0);
}
