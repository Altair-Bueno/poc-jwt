package es.uma.service;

import es.uma.entity.RoleEntity;
import es.uma.entity.SessionEntity;
import es.uma.entity.UserEntity;
import es.uma.model.LoginRequest;
import es.uma.model.RefreshRequest;
import es.uma.model.RegisterRequest;
import es.uma.model.Session;
import es.uma.repository.SessionRepository;
import es.uma.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.beans.factory.annotation.Value;
import org.springframework.security.crypto.password.PasswordEncoder;
import org.springframework.security.oauth2.jwt.JwtClaimsSet;
import org.springframework.security.oauth2.jwt.JwtEncoder;
import org.springframework.security.oauth2.jwt.JwtEncoderParameters;
import org.springframework.stereotype.Service;

import java.time.Instant;
import java.util.stream.Collectors;

@Service
public class AuthService {
    @Autowired
    private UserRepository userRepository;
    @Autowired
    private SessionRepository sessionRepository;
    @Autowired
    private JwtEncoder jwtEncoder;
    @Autowired
    private PasswordEncoder passwordEncoder;

    @Value("${jwt.claims.issuer}")
    private String issuer;

    @Value("${jwt.claims.expire}")
    private Long expire;

    @Value("${jwt.claims.rolesClaims}")
    private String rolesClaims;


    public Session login(LoginRequest loginRequest) {
        var userEntity = userRepository.findUserEntityByUsername(loginRequest.getUsername()).get();

        var sessionEntity = new SessionEntity();
        sessionEntity.setUserEntity(userEntity);
        sessionEntity = sessionRepository.save(sessionEntity);

        var roles = userEntity.getRoleEntityList()
                .stream()
                .map(RoleEntity::getName)
                .collect(Collectors.toList());

        var now = Instant.now();
        var claims = JwtClaimsSet.builder()
                .issuer(issuer)
                .issuedAt(now)
                .expiresAt(now.plusSeconds(expire))
                .subject(userEntity.getId().toString())
                .claim(rolesClaims, roles)
                .build();
        var bearerToken = jwtEncoder.encode(JwtEncoderParameters.from(claims)).getTokenValue();
        var refreshToken = sessionEntity.getId().toString();

        return Session.builder()
                .bearerToken(bearerToken)
                .refreshToken(refreshToken)
                .build();
    }

    public void register(RegisterRequest registerRequest) {
        var username = registerRequest.getUsername();
        var password = passwordEncoder.encode(registerRequest.getPassword());

        var userEntity = new UserEntity();
        userEntity.setUsername(username);
        userEntity.setPassword(password);
        userRepository.save(userEntity);
    }

    public Session refresh(RefreshRequest refreshRequest) {
        // TODO
        return null;
    }

}
