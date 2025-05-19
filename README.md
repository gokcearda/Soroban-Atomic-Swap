# Soroban Atomic Swap Contract

Bu proje, Soroban akıllı sözleşme platformu üzerinde geliştirilmiş bir **atomik takas (atomic swap)** sözleşmesi örneğidir. İki tarafın, aracılara güvenmek zorunda kalmadan, farklı tokenları eş zamanlı olarak takas etmelerine olanak tanır. Takas, her iki tarafın da koşulları karşılandığında "atomik" olarak gerçekleşir; ya her iki transfer de tamamlanır ya da hiçbiri tamamlanmaz.

## Nasıl Çalışır?

Sözleşmenin ana fonksiyonu `swap`'tır. Bu fonksiyon şu parametreleri alır:
-   `a`: Taraf A'nın adresi.
-   `b`: Taraf B'nin adresi.
-   `token_a`: Taraf A'nın sunacağı tokenın adresi.
-   `token_b`: Taraf B'nin sunacağı tokenın adresi.
-   `amount_a`: Taraf A'nın sunacağı `token_a` miktarı.
-   `min_b_for_a`: Taraf A'nın, `amount_a` karşılığında almayı kabul edeceği minimum `token_b` miktarı.
-   `amount_b`: Taraf B'nin sunacağı `token_b` miktarı.
-   `min_a_for_b`: Taraf B'nin, `amount_b` karşılığında almayı kabul edeceği minimum `token_a` miktarı.

İşlem adımları:
1.  Her iki tarafın da sunduğu miktarların, karşı tarafın minimum beklentilerini karşıladığı kontrol edilir.
2.  Her iki tarafın da (`a` ve `b`) bu takası ilgili argümanlarla yetkilendirdiği doğrulanır (`require_auth_for_args`). Bu, kullanıcıların kendi tokenlarını harcamak ve karşılığında diğer tokenı almak için onay verdiğini gösterir.
3.  `move_token` yardımcı fonksiyonu çağrılarak token transferleri gerçekleştirilir:
    -   Taraf A'dan `amount_a` kadar `token_a` sözleşmeye transfer edilir.
    -   Sözleşmeden Taraf B'ye `min_a_for_b` kadar `token_a` transfer edilir.
    -   Eğer `amount_a > min_a_for_b` ise, aradaki fark (`amount_a - min_a_for_b`) sözleşmeden Taraf A'ya iade edilir.
    -   Aynı adımlar `token_b` için Taraf B'den Taraf A'ya doğru tekrarlanır.

Bu yapı, takasın yalnızca her iki tarafın da şartları karşılandığında gerçekleşmesini garanti eder.

## Kurulum ve Başlangıç

Projeyi yerel makinenize klonlayın:

```bash
git clone https://github.com/gokcearda/Soroban-Atomic-Swap.git
cd soroban-atomic-swap