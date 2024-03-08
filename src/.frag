#version 450 core

#define MAX_STEPS 100
#define MAX_DIST 100.0
#define SURF_DIST 0.01

out vec4 fragColor;

uniform vec3 cameraPosition;
uniform vec2 resolution;
uniform float angleX;
uniform float angleY;
uniform float time;

float GetDist(vec3 p) {
    vec4 s = vec4(0.0, 1.0, 5.0, 1.0);
    float sphereDist = length(p - s.xyz) - s.w;
    float planeDist = p.y;
    float d = min(sphereDist, planeDist);
    return d;
}

float RayMarch(vec3 ro, vec3 rd) {
    float dO = 0.0;
    for (int i = 0; i < MAX_STEPS; i++) {
        vec3 p = ro + rd * dO;
        float dS = GetDist(p);
        dO += dS;
        if (dO > MAX_DIST || dS < SURF_DIST) break;
    }
    return dO;
}

vec3 GetNormal(vec3 p) {
    float d = GetDist(p);
    vec2 e = vec2(0.01, 0.0);
    vec3 n = d - vec3(
        GetDist(p - e.xyy),
        GetDist(p - e.yxy),
        GetDist(p - e.yyx)
    );
    return normalize(n);
}

float GetLight(vec3 p) {
    vec3 lightPos = vec3(5.0 * sin(time), 5.0, 6.0 + 5.0 * cos(time));
    vec3 l = normalize(lightPos - p);
    vec3 n = GetNormal(p);
    
    float dif = clamp(dot(n, l), 0.0, 1.0);
    float d = RayMarch(p + n * SURF_DIST * 2.0, l);
    if (d < length(lightPos - p)) dif *= 0.1;
    
    return dif;
}

void main() {
    vec3 col = vec3(0.0);

    vec2 uv = (gl_FragCoord.xy - 0.5 * vec2(resolution.x, resolution.y)) / resolution.y;
    vec3 ro = cameraPosition;
    vec3 rd = normalize(vec3(uv.x, uv.y, 2.0));

    mat3 rotateX = mat3(
        1.0, 0.0, 0.0,
        0.0, cos(angleY), -sin(angleY),
        0.0, sin(angleY), cos(angleY)
    );
    
    mat3 rotateY = mat3(
        cos(angleX), 0.0, sin(angleX),
        0.0, 1.0, 0.0,
        -sin(angleX), 0.0, cos(angleX)
    );
    
    rd = normalize(rotateY * rotateX * rd);

    float d = RayMarch(ro, rd);
    vec3 p = ro + rd * d;

    float dif = GetLight(p);
    col = vec3(dif);

    fragColor = vec4(col, 1.0);
}
