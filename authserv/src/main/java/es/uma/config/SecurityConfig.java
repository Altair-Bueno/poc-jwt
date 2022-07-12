package es.uma.config;

import com.nimbusds.jose.jwk.JWKSet;
import com.nimbusds.jose.jwk.RSAKey;
import com.nimbusds.jose.jwk.source.ImmutableJWKSet;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.config.annotation.web.configuration.EnableWebSecurity;
import org.springframework.security.config.annotation.web.configurers.oauth2.server.resource.OAuth2ResourceServerConfigurer;
import org.springframework.security.config.http.SessionCreationPolicy;
import org.springframework.security.crypto.bcrypt.BCryptPasswordEncoder;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.security.oauth2.jwt.JwtDecoder;
import org.springframework.security.oauth2.jwt.JwtEncoder;
import org.springframework.security.oauth2.jwt.NimbusJwtDecoder;
import org.springframework.security.oauth2.jwt.NimbusJwtEncoder;
import org.springframework.security.oauth2.server.resource.authentication.JwtAuthenticationConverter;
import org.springframework.security.oauth2.server.resource.authentication.JwtGrantedAuthoritiesConverter;
import org.springframework.security.web.SecurityFilterChain;

import java.security.interfaces.RSAPrivateKey;
import java.security.interfaces.RSAPublicKey;

/**
 * Configures Spring Security
 */
@Configuration
@EnableWebSecurity
public class SecurityConfig {
    @Value("${jwt.public.key}")
    private RSAPublicKey rsaPublicKey;
    @Value("${jwt.private.key}")
    private RSAPrivateKey rsaPrivateKey;
    @Value("${jwt.claims.rolesClaims}")
    private String rolesClaims;

    // Extract Spring authorities from the roles claims

    /**
     * Provides a JwtAuthenticationConverter bean. This is used by the
     * oauth2ResourceServer to extract Spring Authorities from the JWT token
     *
     * @return A configured JwtAuthenticationConverter
     */
    @Bean
    public JwtAuthenticationConverter jwtAuthenticationConverter() {
        JwtGrantedAuthoritiesConverter jwtGrantedAuthoritiesConverter = new JwtGrantedAuthoritiesConverter();
        jwtGrantedAuthoritiesConverter.setAuthoritiesClaimName(rolesClaims);
        jwtGrantedAuthoritiesConverter.setAuthorityPrefix("ROLE_");

        JwtAuthenticationConverter jwtAuthenticationConverter = new JwtAuthenticationConverter();
        jwtAuthenticationConverter.setJwtGrantedAuthoritiesConverter(jwtGrantedAuthoritiesConverter);
        return jwtAuthenticationConverter;
    }

    /**
     * Defines the `spring-security` filter chain
     *
     * <ul>
     *     <li>Allows CORS preflights</a></li>
     *     <li>Disables CSRF</li>
     *     <li>Switch session management to stateless</li>
     *     <li>Defines authentication policies for different endpoints</li>
     *     <li>Sets up oauth2ResourceServer for JWT</li>
     * </ul>
     *
     * @param http HttpSecurity builder
     * @return A configured SecurityFilterChain
     * @throws Exception _
     */
    @Bean
    public SecurityFilterChain getSecurityFilterChain(HttpSecurity http) throws Exception {
        return http
                // Enable Spring Security CORS filter. This allows preflight request (HTTP OPTIONS) to
                // pass thought Spring Security Filters
                // Fix 401: https://www.baeldung.com/spring-security-cors-preflight
                .cors().and()
                // Disable CSRF
                .csrf().disable()
                // Set session management to stateless
                .sessionManagement().sessionCreationPolicy(SessionCreationPolicy.STATELESS).and()
                // Set endpoint permissions
                .authorizeRequests()
                .antMatchers(
                        "/auth/login",
                        "/auth/register",
                        "/auth/refresh"
                ).permitAll()
                .anyRequest().authenticated().and()
                // JWT token filter. Requires a JWTDecoder bean
                .oauth2ResourceServer(OAuth2ResourceServerConfigurer::jwt)
                .build();
    }

    /**
     * Provides the JWTEncoder bean
     *
     * @see es.uma.service.AuthService
     * @return A configured JWT encoder
     */
    @Bean
    public JwtEncoder getJwtEncoder() {
        var jwk = new RSAKey.Builder(this.rsaPublicKey)
                .privateKey(this.rsaPrivateKey)
                .build();
        var jwks = new ImmutableJWKSet<>(new JWKSet(jwk));
        return new NimbusJwtEncoder(jwks);
    }

    /**
     * Provides the JWTDecoder bean. This will be used by the
     * oauth2ResourceServer
     *
     * @return A configured JWT decoder
     */
    @Bean
    public JwtDecoder getJwtDecoder() {
        return NimbusJwtDecoder.withPublicKey(this.rsaPublicKey).build();
    }

    /**
     * Provides the Password encoder bean.
     *
     * @see es.uma.service.AuthService
     * @return A configured password encoder
     */
    @Bean
    public PasswordEncoder getPasswordEncoder() {
        return new BCryptPasswordEncoder(11);
    }
}
