def test_imports():
    try:
        from hummingbot.connector.derivative.decibel_perpetual import auth
        assert True
    except ImportError:
        assert False, "Auth-moduulia ei löytynyt!"
