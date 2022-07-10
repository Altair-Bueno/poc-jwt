package es.uma.model.request;

import com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Value;

@Value
public class RefreshRequest {
    @JsonProperty("refresh_token")
    String refreshToken;
    String username;
}