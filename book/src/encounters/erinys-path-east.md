<!-- area-id: erinys-path-east -->
### Erinys Path (East)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=cherubi">Cherubi</a> | 20% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="cherubi" /> |
| <a href="../pokemon-lookup.html?q=falinks">Falinks</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="falinks" /> |
| <a href="../pokemon-lookup.html?q=gastly">Gastly</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="gastly" /> |
| <a href="../pokemon-lookup.html?q=houndour">Houndour</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="houndour" /> |
| <a href="../pokemon-lookup.html?q=kabuto">Kabuto</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="kabuto" /> |
| <a href="../pokemon-lookup.html?q=mimikyu">Mimikyu</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="mimikyu" /> |
| <a href="../pokemon-lookup.html?q=minior-green-core">Minior Green Core</a> | 4% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="minior-green-core" /> |
| <a href="../pokemon-lookup.html?q=minior-yellow-core">Minior Yellow Core</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="minior-yellow-core" /> |
| <a href="../pokemon-lookup.html?q=onix">Onix</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="onix" /> |
| <a href="../pokemon-lookup.html?q=spinarak">Spinarak</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="spinarak" /> |
| <a href="../pokemon-lookup.html?q=fomantis">Fomantis</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="fomantis" /> |
| <a href="../pokemon-lookup.html?q=minior-violet-core">Minior Violet Core</a> | — | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="minior-violet-core" /> |
| <a href="../pokemon-lookup.html?q=salandit">Salandit</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="salandit" /> |
| <a href="../pokemon-lookup.html?q=skorupi">Skorupi</a> | — | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="skorupi" /> |
| <a href="../pokemon-lookup.html?q=seel">Seel</a> | — | — | 30% | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="seel" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | 10% | — | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=spheal">Spheal</a> | — | — | 60% | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="spheal" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | — | — | — | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=poliwag">Poliwag</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="poliwag" /> |
| <a href="../pokemon-lookup.html?q=corphish">Corphish</a> | — | — | — | — | — | 20% | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="corphish" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | — | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="erinys-path-east" data-species="remoraid" /> |

<script>
(function() {
  const STORAGE_KEY = 'lazarusEncounterChecks';
  function loadState() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');
    } catch (_) {
      return {};
    }
  }
  function saveState(state) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch (_) {}
  }
  const state = loadState();
  const checkboxes = document.querySelectorAll('.encounter-check');
  checkboxes.forEach(cb => {
    const key = `${cb.dataset.area}|${cb.dataset.species}`;
    cb.checked = !!state[key];
    cb.addEventListener('change', () => {
      if (cb.checked) {
        state[key] = true;
      } else {
        delete state[key];
      }
      saveState(state);
    });
  });
})();
</script>

