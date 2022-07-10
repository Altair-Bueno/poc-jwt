package es.uma.model;

import lombok.Builder;
import lombok.Value;

@Value
@Builder
public class Session {
    String bearerToken;
    String refreshToken;
}
