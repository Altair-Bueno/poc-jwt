package es.uma.service;

import es.uma.entity.RoleEntity;
import es.uma.entity.SessionEntity;
import es.uma.entity.UserEntity;
import es.uma.model.request.LoginRequest;
import es.uma.model.request.RefreshRequest;
import es.uma.model.request.RegisterRequest;
import es.uma.model.response.Session;
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
import java.util.UUID;
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

        return sessionEntityToResponse(sessionEntity);
    }


    public Session refresh(RefreshRequest refreshRequest) {
        var id = UUID.fromString(refreshRequest.getRefreshToken());
        var sessionEntity = sessionRepository.findById(id).get();
        if (!sessionEntity.getUserEntity().getUsername().equals(refreshRequest.getUsername()))
            throw new RuntimeException("Usernames don't match");

        return sessionEntityToResponse(sessionEntity);
    }

    private Session sessionEntityToResponse(SessionEntity sessionEntity) {
        var userEntity = sessionEntity.getUserEntity();
        var roles = userEntity.getRoleEntityList()
                .stream()
                .map(RoleEntity::getName)
                .collect(Collectors.toList());

        var now = Instant.now();
        // https://jwt.io
        var jwtClaimsSet = JwtClaimsSet.builder()
                .issuer(issuer)
                .issuedAt(now)
                .expiresAt(now.plusSeconds(expire))
                .subject(userEntity.getId().toString())
                .claim(rolesClaims, roles)
                .build();
        var accessToken = jwtEncoder.encode(JwtEncoderParameters.from(jwtClaimsSet)).getTokenValue();
        var refreshToken = sessionEntity.getId();

        return Session.builder()
                .refreshToken(refreshToken)
                .accessToken(accessToken)
                .expiresIn(expire)
                .tokenType("Bearer")
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
}
