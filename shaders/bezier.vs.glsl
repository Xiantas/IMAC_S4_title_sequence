#version 330 core
layout(location=0) in int aTruc;
layout(location=1) in vec2[4] aControlPoints;

out int vIntensity;

uniform int uTotalVerticesCount;

vec2 polynomeBezier(float t){
    return aControlPoints[0]*pow(1-t,3) + 
           aControlPoints[1]*3*t*pow(1-t,2) + 
           aControlPoints[2]*3*pow(t,2)*(1-t) + 
           aControlPoints[3]*pow(t,3);
}

vec2 deriveBezier(float t){
    return aControlPoints[0]*(-3)*pow(1-t,2) +
                  aControlPoints[1]*(3*pow(1-t,2)-6*t*(1-t)) + 
                  aControlPoints[2]*(6*t*(1-t)-3*pow(t,2)) +
                  aControlPoints[3]*3*pow(t,2);
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