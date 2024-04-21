#version 460 core
layout(location=0) in int aTruc;
layout(location=1) in vec2 aP0;
layout(location=2) in vec2 aP1;
layout(location=3) in vec2 aP2;
layout(location=4) in vec2 aP3;

out float vIntensity;

const vec2 bP0 = vec2(-0.5, 0.5);
const vec2 bP1 = vec2(0.5, 0.5);
const vec2 bP2 = vec2(-0.5, -0.5);
const vec2 bP3 = vec2(0.5, -0.5);

uniform int uTotalVerticesCount;

const vec2 corones[4] = vec2[4](
    vec2(0.5, 0.5),
    vec2(-0.5, 0.5),
    vec2(0.5, -0.5),
    vec2(-0.5, -0.5)
);

vec2 polynomeBezier(float t){
    return bP0*pow(1-t,3) + 
           bP1*3*t*pow(1-t,2) + 
           bP2*3*pow(t,2)*(1-t) + 
           bP3*pow(t,3);
}


vec2 deriveBezier(float t){
    return bP0*(-3)*pow(1-t,2) +
                  bP1*(3*pow(1-t,2)-6*t*(1-t)) + 
                  bP2*(6*t*(1-t)-3*pow(t,2)) +
                  bP3*3*pow(t,2);
}

vec2 normalBezier(float t){
    vec2 normal = deriveBezier(t);
    return vec2(normal.y*pow(-1,gl_VertexID%2),normal.x*pow(-1,(gl_VertexID%2)+1));
}

vec2 polyBez(float t) {
    return (-bP0 + 3*bP1 - 3*bP2 + bP3) * t*t*t +
        (3*bP0 - 6*bP1 + 3*bP2) * t*t +
        (-3*bP0 + 3*bP1) * t + bP0;
}

vec2 deriBez(float t) {
    return 3*(-bP0 + 3*bP1 - 3*bP2 + bP3) * t*t +
        2*(3*bP0 - 6*bP1 + 3*bP2) * t +
        (-3*bP0 + 3*bP1);
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

void main(){
    float t = float(gl_VertexID) / float(uTotalVerticesCount);
    if(gl_VertexID == 0 || gl_VertexID == uTotalVerticesCount-1){
        vIntensity = 0.0;
    }
    else if (gl_VertexID % 2 == 0) {
        vIntensity = 1.0;
    } else {
        vIntensity = -1.0;
    }
    vec2 pos = polyBez(t) + normaBez(t);

    if (length(pos) > 1.0 || length(pos) < 0.05) {
        pos = normalize(pos);
    }

    gl_Position = vec4(pos, 0.0, 1.0);
    vec4(corones[gl_VertexID%4], 0.0, 1.0);

}
