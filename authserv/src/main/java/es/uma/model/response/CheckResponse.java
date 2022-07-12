package es.uma.model.response;

import lombok.Builder;
import lombok.Value;

import java.util.Collection;

@Value
@Builder
public class CheckResponse {
    String name;
    Collection<String> authorities;
}
