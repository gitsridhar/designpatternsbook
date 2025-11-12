class Kitchen:
    _instance = None

    def __new__(cls, *args, **kwargs):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
        return cls._instance

    def __init__(self, value):
        if not hasattr(self, '_initialized'):
            self.value = value
            self._initialized = True

def main():
    s1 = Kitchen("First instance")
    s2 = Kitchen("Second instance")

    print(s1 is s2)  # Output: True
    print(s1.value) # Output: First instance
    print(s2.value) # Output: First instance (since it's the same instance)
    
if __name__ == "__main__":
    main()