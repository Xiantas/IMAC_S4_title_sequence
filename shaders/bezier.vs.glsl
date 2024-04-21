#version 330 core
layout(location=0) in int aTruc;
layout(location=1) in vec2 aP0;
layout(location=2) in vec2 aP1;
layout(location=3) in vec2 aP2;
layout(location=4) in vec2 aP3;

out int vIntensity;

uniform int uTotalVerticesCount;

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

void main(){
    float t = float(gl_VertexID) / float(uTotalVerticesCount);
    if(gl_VertexID == 0 || gl_VertexID == uTotalVerticesCount){
        vIntensity = 0;
    }
    else{
        vIntensity = int(pow(-1,gl_VertexID%2));
    }
    gl_Position = vec4(normalBezier(t),0.,1.);

}
