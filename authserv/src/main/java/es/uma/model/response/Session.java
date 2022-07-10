package es.uma.model.response;

import lombok.Builder;
import lombok.Value;

import java.util.UUID;

@Value
@Builder
public class Session {
    String bearerToken;
    UUID refreshToken;
}
