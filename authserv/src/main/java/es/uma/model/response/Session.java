package es.uma.model.response;

import lombok.Builder;
import lombok.Value;

@Value
@Builder
public class Session {
    String bearerToken;
    String refreshToken;
}
